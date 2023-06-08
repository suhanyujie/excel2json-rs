use calamine::{open_workbook, Reader, Xlsx};

fn convert_to_json_file() {
    let mut excel: Xlsx<_> = open_workbook("file.xlsx").unwrap();
    if let Some(Ok(r)) = excel.worksheet_range("Sheet1") {
        for row in r.rows() {
            println!("row={:?}, row[0]={:?}", row, row[0]);
        }
    }
}
