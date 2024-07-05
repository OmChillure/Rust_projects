use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_cors::Cors;
use actix_multipart::Multipart;
use flate2::write::GzEncoder;
use flate2::Compression;
use futures_util::stream::{StreamExt, TryStreamExt};
use std::fs::File;
use std::io::{BufReader, copy, Write};
use std::time::Instant;
use uuid::Uuid;

// fn main () {
    // if args().len() != 3 {
    //     println!("Usage: `source` `target`")
    // }

    // let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    // let mut output = File::create(args().nth(2).unwrap()).unwrap();
    // let mut encoder = GzEncoder::new(output, Compression::default());
    // let start = Instant::now();
    // copy(&mut input, &mut encoder).unwrap();
    // let output = encoder.finish().unwrap();
    // println!("Output: {}", output.metadata().unwrap().len());
    // println!("Elapsed time: {:?}", start.elapsed());
// }
async fn upload_and_compress(mut payload: Multipart) -> impl Responder {
    let temp_file_path = format!("./{}.tmp", Uuid::new_v4());
    let compressed_file_path = format!("./{}.gz", &temp_file_path);

    let temp_file_path_clone = temp_file_path.clone();
    let mut file = web::block(move || File::create(&temp_file_path_clone)).await.unwrap().unwrap();

    while let Ok(Some(mut field)) = payload.try_next().await {
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            file = web::block(move || file.write_all(&data).map(|_| file)).await.unwrap().unwrap();
        }
    }
    
    let mut input = BufReader::new(File::open(&temp_file_path).unwrap());
    let output = File::create(&compressed_file_path).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!("Output: {}", output.metadata().unwrap().len());
    println!("Elapsed time: {:?}", start.elapsed()); 

    std::fs::remove_file(&temp_file_path).unwrap();

    HttpResponse::Ok().body(format!("Compressed file created at: {}", compressed_file_path))
}

#[actix_web::main]
async fn main() -> std::io::Result <()> {
    HttpServer::new(|| {
        let cors = Cors::default()
           .allow_any_origin()
           .allow_any_header()
           .allow_any_method()
           .max_age(3600);

           App::new()
           .wrap(cors)
           .route("/upload", web::post().to(upload_and_compress))
    })
    .bind("127.0.0.1:8080")? 
    .run()
    .await
}

