/// Converters between csv &str data and ndarray.
use crate::prelude::*;

/// parse csv &str data to Array1<f64> with specified length.s
pub fn csv_to_1darray(csv_str: &str, len: usize) -> Result<Array1<f64>, Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(csv_str.as_bytes());
    let mut signal = vec![0.; len];
    let values: Vec<f64> = reader.deserialize().nth(0).unwrap()?;
    if values.len() < len {
        return Err(From::from("the number of csv data is less than needed(len)."));
    }
    for i in 0..len {
        signal[i] = values[i];
    }

    Ok(Array::from(signal))
}

/// parse 2d csv &str data to Array2<f64> with specified shape.
/// csv data's row size need to be larger than shape.0,and its column size need to be larger than shape.1.
///
/// # Erros
///
/// ```
/// let csv = "\
/// 1.,2.,
/// 2.2,1.5,
/// ";
///
/// otr_tgi::csv_converter::csv_to_2darray(csv, (2, 3)).unwrap();
///
/// ```
pub fn csv_to_2darray(csv_str: &str, shape: (usize, usize)) -> Result<Array2<f64>, Box<dyn Error>> {
    let mut signals: Array2<f64> = ArrayBase::zeros(shape);
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(csv_str.as_bytes());

    let mut unfilled_row = 0;
    for (row, signal) in reader.deserialize::<Vec<f64>>().enumerate() {
        let signal = signal?;

        //check row size and column size.
        if row >= shape.0 {
            break;
        }
        unfilled_row = row + 1;
        if signal.len() < shape.1 {
            return Err(From::from(
                "a csv data's row size is less than needed(shape.1).",
            ));
        }

        for column in 0..shape.1 {
            signals[[row, column]] = signal[column];
        }
    }
    //check the number of rows.
    if unfilled_row < shape.0 {
        return Err(From::from(
            "the number of rows of csv data is less than needed(shape.0).",
        ));
    }

    Ok(signals)
}

pub fn save_1darray(arr: &Array1<f64>, path: &str) -> Result<(), Box<dyn Error>> {
    let f = std::fs::File::open(path)?;

    let mut writer = csv::Writer::from_writer(f);

    writer.serialize(arr.to_vec())?;

    Ok(())
}