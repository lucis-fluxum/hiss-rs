use pyo3::prelude::*;

fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        let sys = py.import("sys")?;
        let path: Vec<String> = sys.get("path")?.extract()?;
        println!("{:?}", path);
        Ok(())
    })
}
