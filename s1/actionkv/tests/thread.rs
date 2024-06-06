#![cfg_attr(
    debug_assertions,
    allow(
        unused,
        dead_code,
        unused_imports,
        unused_variables,
        unused_assignments,
        non_snake_case
    )
)]
use num_cpus;
use std::sync::{Arc, Mutex};
use std::time::{Duration as stdDuration, Instant};
use std::{io, thread};
use thread_control::*;
use tokio::{
    self,
    sync::broadcast,
    sync::mpsc,
    task::JoinSet,
    time::{self, Duration},
};

pub fn tokio_async() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        println!("Hello from tokio!");
        rt.spawn(async {
            println!("Hello from a tokio task!");
            println!("in spawn")
        })
        .await
        .unwrap();
    });
    rt.spawn_blocking(|| println!("in spawn_blocking"));
}

mod tests {
    use super::*;
    #[test]
    fn test_thread() {
        let count = thread::available_parallelism().unwrap().get();
        println!("count={:?}", count);
        assert!(count > 1_usize);

        let num = num_cpus::get();
        println!("cpu number={:?}", num);

        let (flat, control) = make_pair();
    }
    #[test]
    fn test_scope_spawn() {
        // tokio::runtime::Builder::new_multi_thread()
        //     .enable_all()
        //     .build()
        //     .unwrap()
        //     .block_on(async {
        //         // config::global_configure().await;
        //     })
    }
    #[test]
    fn test_start_tokio() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            println!("Hello from tokio!");
            rt.spawn(async {
                println!("Hello from a tokio task!");
                println!("in spawn")
            })
            .await
            .unwrap();
        });
        rt.spawn_blocking(|| println!("in spawn_blocking"));
    }
    #[test]
    fn test_thin_ptr() {
        // use std::mem::{size_of, size_of_val};
        // let size_of_ptr = size_of::<*const ()>();
        // let box_five = Box::new(5);
        // let box_slice = Box::<[i32]>::new_zeroed_slice(5);
        // assert_eq!(size_of_ptr, size_of_val(&box_five));
        // assert_eq!(size_of_ptr * 2, size_of_val(&box_slice));
        // let five = ThinBox::new(5);
        // let thin_slice = ThinBox::<[i32]>::new_unsize([1, 2, 3, 4]);
        // assert_eq!(size_of_ptr, size_of_val(&five));
        // assert_eq!(size_of_ptr, size_of_val(&thin_slice));
    }
    // todo 验证锁的poison
    #[test]
    fn test_sleep() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            println!("Hello from tokio!");
            let mut handlers = Vec::with_capacity(5);
            for i in 0..5 {
                let h = tokio::spawn(async {
                    let start = Instant::now();
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    let duration = start.elapsed();
                    println!("stopped {:?}", duration);
                });
                handlers.push(h);
            }
            for h in handlers.into_iter() {
                h.await;
            }
        });
    }
    #[test]
    fn test_lock() {
        let a = Mutex::new(100);
        let b = a.lock().unwrap();
        *(a.lock().unwrap()) += 1;
        *(a.lock().unwrap()) += 10;
        *(a.lock().unwrap()) += 100;
        println!("a = {:?}", a);
    }
    #[test]
    fn test_broadcast() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let (tx, mut rx1) = broadcast::channel(16);
            // 生成一个新的receiver
            let mut rx2 = tx.subscribe();
            tokio::spawn(async move {
                let r1 = rx1.recv().await.unwrap();
                let r2 = rx1.recv().await.unwrap();
                println!("r1 = {:?}", r1);
                println!("r2 = {:?}", r2);
            });
            tokio::spawn(async move {
                assert_eq!(rx2.recv().await.unwrap(), 10);
                assert_eq!(rx2.recv().await.unwrap(), 20);
            });
            tx.send(10).unwrap();
            tx.send(20).unwrap();
        });
    }
    async fn sleep(n: u64) -> u64 {
        time::sleep(Duration::from_secs(n)).await;
        n
    }
    #[test]
    fn test_select() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            // 这里进入sleep(5),每次都是sleep 3 结束
            tokio::select! {
              v = sleep(5) => println!("sleep 5 secs, branch 1 done: {}", v),
              v = sleep(3) => println!("sleep 3 secs, branch 2 done: {}", v),
              else =>{
                println!("exit loop");
              }
            };
            println!("select! done");

            let s5 = sleep(5);
            let s3 = sleep(3);

            let f = Instant::now();
            tokio::join!(s5, s3);
            println!("s3 s5 done :{:?}", f.elapsed());
            // let s5 = sleep(5);
            // let s3 = sleep(3);
            // tokio::pin!(s5);
            // tokio::pin!(s3);
            // loop {
            //     tokio::select! {
            //         _ = &mut s3 => {
            //             println!("s3 end");
            //         },
            //         // _ = &mut s5=>{
            //         //     println!("s5 end");
            //         // },
            //         else=>{
            //             println!("select! loop done");
            //             break;
            //         },
            //     }
            // }

            // loop {
            //     tokio::select! {
            //         v = sleep(5) => println!("sleep 5 secs, branch 1 done: {}", v),
            //         v = sleep(3) => println!("sleep 3 secs, branch 2 done: {}", v),
            //         else =>{
            //           println!("exit loop");
            //           break;
            //         }
            //       };
            //       println!("select! done");
            // }
            let mut set = JoinSet::new();
            for i in 1..=5 {
                set.spawn(async move {
                    sleep(i).await;
                });
            }
            // 5个任务都成功才推出
            while let Some(ret) = set.join_next().await {
                println!("task join set got end ret = {:?}", ret);
            }
        });
    }
}

// todo 使用tokio scope thread和 crossbeam的scope thread，同时发起2个请求指定不同的任务，并等待结束
// todo 定义一个ticker 定时执行一个任务
// 同时发起多个请求，任何一个有结果返回， join!
// 同时执行多个任务，等待所有的结果返回  select!
