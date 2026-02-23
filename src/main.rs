use std::sync::mpsc;
use std::thread;
use std::time::{Instant};

fn main() {
    let (tx, rx) = mpsc::channel::<String>();
    // Creates a multi-producer, single-consumer channel.
    // tx = Sender<u64>
    // rx = Receiver<u64>

    let producers = 3;

    // Spawn Producers
    for id in 0..producers {
        let tx_clone = tx.clone();
        // Clone sender so multiple threads can send to same channel

        thread::spawn(move || {

            // 1 million msgs send
            for _ in 0..1_000_000 {
                tx_clone.send("hello__bro the greatest of all time".to_string()).unwrap();
            }
        });

        println!("Producer {} started", id);
    }

    drop(tx); 
    // Drops the original sender.
    // Important because channel closes ONLY when all senders are dropped.
    // But producers still hold cloned senders.

    // Consumer
    thread::spawn(move || {
        let mut count: u64 = 0;
        let start = Instant::now(); // Note intila timing 
    
        while let Ok(_v) = rx.recv() { // wait for it to get completed 
            
            count += 1;
        }
    

        let elapsed = start.elapsed().as_secs_f64(); // calulated the elapsed time 
        let throughput = count as f64 / elapsed; // and final throughput
    
        println!("Total received: {}", count);
        println!("Elapsed time: {:.3} seconds", elapsed);
        println!("Throughput: {:.0} msgs/sec", throughput);
    })
    .join()
    .unwrap();
}



// Note to programmer:
//
// In this demo, throughput is calculated at the Receiver endpoint,
// meaning we measure how many messages were successfully processed.
//
// Measuring across the full Sender → Channel → Receiver pipeline
// provides a more complete end to end throughput perspective.
//
// In real-world production systems, throughput metrics may differ
// depending on where measurement occurs (send-side, receive-side,
// network boundary, or internal queue).
//
// Production throughput is influenced by factors such as:
// - Thread scheduling
// - Lock contention
// - Memory allocation
// - Context switching
// - Backpressure
//
// This example is a simplified educational benchmark intended for demonstration purposes.