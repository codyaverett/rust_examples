use async_std::fs::File;
use async_std::io::{BufReader, Result};
use async_std::prelude::*;
use async_std::task;

async fn read_file(file_path: &str) -> Result<Vec<u8>> {
    let file = File::open(file_path).await?;
    let mut reader = BufReader::new(file);

    let mut contents = vec![];
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = reader.read(&mut buffer).await?;
        if bytes_read == 0 {
            break;
        }

        contents.extend_from_slice(&buffer[..bytes_read]);
    }

    Ok(contents)
}

async fn get(file_path: &str) -> Result<()> {
    match read_file(file_path).await {
        Ok(contents) => {
            let contents_str = String::from_utf8_lossy(&contents);
            println!("{}", contents_str);
        }
        Err(error) => {
            eprintln!("Error reading file: {}", error);
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    task::block_on(async {
        let future = get("./src/toast.yml");

        let result = future.await;

        println!("Future content: {:?}", result);
    });

    Ok(())
}
