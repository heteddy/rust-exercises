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
use std::{io, thread};
use thread_control::*;
use tokio;

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
    
}

// todo 使用tokio scope thread和 crossbeam的scope thread，同时发起2个请求指定不同的任务，并等待结束
