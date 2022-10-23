use tokio_uring::fs::File;

#[test]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    ::tokio_uring::start(async {
        // Open a file
        let file = File::open("tests/io_uring.rs").await?;

        let buf = vec![0; 4096];
        // Read some data, the buffer is passed by ownership and
        // submitted to the kernel. When the operation completes,
        // we get the buffer back.
        let (res, buf) = file.read_at(buf, 0).await;
        let n = res?;

        // Display the contents
        println!("{:?}", &buf[..n]);

        Ok(())
    })
}
