use std::{
    sync::{Arc, Condvar, Mutex},
    time::{Duration, Instant},
};

use log::info;
use reqwest::{Client, ClientBuilder};
use tokio::runtime::Builder;

#[derive(Debug, Copy, Clone)]
enum Task {
    Io(usize),
    Cpu(usize),
}

impl Task {
    fn label(&self) -> String {
        match self {
            Self::Io(i) => format!("io-{i}"),
            Self::Cpu(i) => format!("cpu-{i}"),
        }
    }
}

async fn io_task(client: Client, task: Task) {
    let now = Instant::now();
    let label = task.label();
    info!("{label} begin");
    let resp = client
        .get(format!("http://127.0.0.1:8080/{}", task.label()))
        .send()
        .await;
    let _ = resp.unwrap().text().await.unwrap();

    match task {
        Task::Io(i) => {
            info!("io-{i} cost:{:?}", Instant::now().duration_since(now));
        }
        Task::Cpu(i) => {
            info!("cpu-{i} cost:{:?}", Instant::now().duration_since(now));
            // Use sleep to block tokio worker thread
            std::thread::sleep(Duration::from_millis(200));
        }
    }
}

fn main() {
    env_logger::builder().format_timestamp_millis().init();

    let http_client = ClientBuilder::new()
        // change 0 to disable conn pool
        .pool_max_idle_per_host(4)
        .connection_verbose(true)
        .build()
        .unwrap();
    let io = Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .thread_name("io")
        .build()
        .unwrap();
    let io = Arc::new(io);

    let io_client = http_client.clone();
    let wait_start = Arc::new((Mutex::new(false), Condvar::new()));
    let io_wait = wait_start.clone();
    let cpu_wait = wait_start.clone();

    std::thread::spawn(move || {
        {
            // wait start
            let (lock, cvar) = &*io_wait;
            let mut started = lock.lock().unwrap();
            while !*started {
                started = cvar.wait(started).unwrap();
            }
            println!("IO tasks started...");
        }

        for i in 0..10 {
            std::thread::sleep(Duration::from_millis(10));
            let hc = io_client.clone();
            io.spawn(async move {
                io_task(hc, Task::Io(i)).await;
            });
        }
        std::thread::sleep(Duration::from_secs(1000000));
    });

    let cpu = Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .thread_name("cpu")
        .build()
        .unwrap();
    let cpu = Arc::new(cpu);

    std::thread::spawn(move || {
        {
            // wait start
            let (lock, cvar) = &*cpu_wait;
            let mut started = lock.lock().unwrap();
            while !*started {
                started = cvar.wait(started).unwrap();
            }
            println!("CPU tasks started...");
        }

        for i in 0..10 {
            std::thread::sleep(Duration::from_millis(10));
            let hc = http_client.clone();
            cpu.spawn(async move {
                io_task(hc, Task::Cpu(i)).await;
            });
        }
        std::thread::sleep(Duration::from_secs(1000000));
    });

    {
        let (lock, cvar) = &*wait_start;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_all();
        println!("Start running...");
    }

    std::thread::sleep(Duration::from_secs(1000000));
}
