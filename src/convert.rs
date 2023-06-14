use calamine::{open_workbook, Reader, Xlsx};

fn convert_to_json_file() {
    let default_str = "";
    let mut excel: Xlsx<_> = open_workbook("./example/data/table1.xlsx").unwrap();
    if let Some(Ok(r)) = excel.worksheet_range("Sheet1") {
        let mut key_arr: Vec<String> = vec![];
        let mut col_name_arr: Vec<String> = vec![];
        let mut col_alias_arr: Vec<String> = vec![];
        let mut is_arr_obj_arr: Vec<String> = vec![];
        let mut is_arr_obj_arr2: Vec<String> = vec![];
        for (i, row) in r.rows().enumerate() {
            match i {
                0 => {
                    for cell in row {
                        key_arr.push(cell.as_string().unwrap());
                    }
                }
                1 => {
                    for cell in row {
                        is_arr_obj_arr.push(cell.as_string().unwrap());
                    }
                }
                2 => {
                    for cell in row {
                        col_name_arr.push(cell.as_string().unwrap());
                    }
                }
                3 => {
                    for cell in row {
                        is_arr_obj_arr2.push(cell.as_string().unwrap());
                    }
                }
                4 => {
                    for cell in row {
                        col_alias_arr.push(cell.as_string().unwrap());
                    }
                }
                _ => {
                    println!("row={:?}, row[0]={:?}", row, row[0]);
                }
            }
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
