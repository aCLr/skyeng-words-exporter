use anyhow::Result;
use entity::words::Model as Word;
use xlsxwriter::Workbook;

pub fn export_to_xlsx(destination: &str, words: &Vec<Word>) -> Result<()> {
    let wb = Workbook::new(destination);

    let mut sheet = wb.add_worksheet(None)?;

    sheet.write_string(0, 0, "русский", None)?;
    sheet.write_string(0, 1, "examples", None)?;
    sheet.write_string(0, 2, "перевод", None)?;

    for (i, word) in words.into_iter().enumerate() {
        let i = (i + 1) as u32;

        sheet.write_string(i, 0, word.text.as_str(), None)?;
        sheet.write_string(i, 1, word.examples.as_str(), None)?;
        sheet.write_string(i, 2, word.translation.as_str(), None)?;
    }
    Ok(())
}
