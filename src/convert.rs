use calamine::{open_workbook, Reader, Xlsx};

fn convert_to_json_file() {
    let mut excel: Xlsx<_> = open_workbook("./example/data/table1.xlsx").unwrap();
    if let Some(Ok(r)) = excel.worksheet_range("Sheet1") {
        for row in r.rows() {
            println!("row={:?}, row[0]={:?}", row, row[0]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_json_file1() {
        convert_to_json_file();
    }
}
