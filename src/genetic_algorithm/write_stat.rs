use std::{
    fs::{create_dir_all, File},
    io::{BufWriter, Write},
};

use ron::ser::{to_string_pretty, PrettyConfig};

#[derive(Default)]
pub struct InstanceStats {
    pub generation_count: usize,
    pub best_fitness: f32,
    pub best_fitness_sum: f32,
}

impl InstanceStats {
    pub fn write(&self, file_path: String) {
        let path = std::path::Path::new(&file_path);
        let prefix = path.parent().unwrap();

        create_dir_all(prefix).unwrap();
        let mut stream = BufWriter::new(File::create(path).unwrap());
        let _ = writeln!(
            stream,
            "GenerationCount: {}, Best Fitness: {}, Avg: {}",
            self.generation_count,
            self.best_fitness,
            self.best_fitness_sum / self.generation_count as f32
        );
    }
}

pub fn write_stat(file_path: String, best: f32, median: f32, worst: f32, avg: f32, std_dev: f32) {
    let path = std::path::Path::new(&file_path);
    let prefix = path.parent().unwrap();

    create_dir_all(prefix).unwrap();
    let buffer = File::create(path).unwrap();
    let mut stream = BufWriter::new(buffer);
    stream.write_all(b"Best: ").unwrap();
    stream
        .write_all(
            to_string_pretty(&best, PrettyConfig::default())
                .unwrap()
                .as_bytes(),
        )
        .unwrap();
    stream.write_all(b"\n").unwrap();
    stream.write_all(b"Worst: ").unwrap();
    stream
        .write_all(
            to_string_pretty(&worst, PrettyConfig::default())
                .unwrap()
                .as_bytes(),
        )
        .unwrap();
    stream.write_all(b"\n").unwrap();
    stream.write_all(b"Median: ").unwrap();
    stream
        .write_all(
            to_string_pretty(&median, PrettyConfig::default())
                .unwrap()
                .as_bytes(),
        )
        .unwrap();
    stream.write_all(b"\n").unwrap();
    stream.write_all(b"Avg: ").unwrap();
    stream
        .write_all(
            to_string_pretty(&avg, PrettyConfig::default())
                .unwrap()
                .as_bytes(),
        )
        .unwrap();
    stream.write_all(b"\n").unwrap();
    stream.write_all(b"Std. Dev.: ").unwrap();
    stream
        .write_all(
            to_string_pretty(&std_dev, PrettyConfig::default())
                .unwrap()
                .as_bytes(),
        )
        .unwrap();
    stream.write_all(b"\n").unwrap();
    stream.flush().unwrap();
}
