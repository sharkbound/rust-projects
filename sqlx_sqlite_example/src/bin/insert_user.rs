use async_std::task;

fn main() {
    task::block_on(amain())
}

async fn amain() {
    println!("hello world!")
}