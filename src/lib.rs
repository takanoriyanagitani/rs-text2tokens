use std::io;

use io::BufWriter;
use io::Write;

use io::BufRead;

use unicode_segmentation::UnicodeSegmentation;

pub fn str2tokens2writer<W>(original: &str, wtr: &mut W) -> Result<(), io::Error>
where
    W: Write,
{
    let words = original.unicode_words();
    for token in words {
        writeln!(wtr, "{token}")?;
    }
    Ok(())
}

pub fn strings2tokens2writer<I, W>(strings: I, mut wtr: W) -> Result<(), io::Error>
where
    I: Iterator<Item = Result<String, io::Error>>,
    W: Write,
{
    for rline in strings {
        let line: String = rline?;
        str2tokens2writer(&line, &mut wtr)?;
    }
    wtr.flush()
}

pub fn stdin2strings2tokens2stdout() -> Result<(), io::Error> {
    let o = io::stdout();
    let mut ol = o.lock();
    strings2tokens2writer(io::stdin().lock().lines(), BufWriter::new(&mut ol))?;
    ol.flush()
}
