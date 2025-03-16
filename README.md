# Tutorial Pemrograman Lanjut
## Fransisca Ellya Bunaren - 2306152286

## Commit 1 Reflection
> what is inside the handle_connection

Fungsi `handle_connection` dalam Rust digunakan untuk menangani koneksi TCP dari klien dengan membaca permintaan HTTP yang dikirim. Fungsi ini membungkus koneksi stream dalam BufReader agar dapat membaca data secara lebih efisien. Selanjutnya, data dibaca baris per baris menggunakan `.lines()` dan dikumpulkan dalam sebuah vektor setelah difilter dengan .`take_while(|line| !line.is_empty())`, yang memastikan hanya header HTTP yang diambil hingga baris kosong ditemukan. Dengan cara ini, server dapat memperoleh informasi seperti metode HTTP (GET, POST), path URL, serta berbagai header lainnya seperti User-Agent dan Host. Data yang dikumpulkan ini kemudian dapat digunakan untuk memproses permintaan klien dan menentukan respons yang sesuai.