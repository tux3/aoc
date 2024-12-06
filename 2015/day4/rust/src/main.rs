use anyhow::Result;
use md5::Digest;

fn main() -> Result<()> {
    // This solves part 2
    // Part 1 is the same, but either doing starts_with on the hex, or checking hash[3] < 0x10
    let input = "yzbqklnj";
    let mut i = 0;
    let good = loop {
        let hash = md5::Md5::new()
            .chain_update(format!("{input}{i}").as_bytes())
            .finalize()
            .to_vec();
        if hash.starts_with(&[0, 0, 0]) {
            break i;
        }
        i += 1
    };

    println!("Result: {input}{good}");
    Ok(())
}
