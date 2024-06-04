/*
 * SPDX-FileCopyrightText: 2023 Inria
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 * SPDX-FileCopyrightText: 2023 Tommaso Fontana
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use anyhow::Result;
use epserde::prelude::*;
use std::io::prelude::*;
use std::io::BufReader;
use sux::prelude::*;

#[test]
fn test_rear_coded_list_100() -> Result<()> {
    test_rear_coded_list("tests/data/wordlist.100")
}

#[cfg(feature = "slow")]
#[test]
fn test_rear_coded_list_10000() -> Result<()> {
    test_rear_coded_list("tests/data/wordlist.10000")
}

fn test_rear_coded_list(path: impl AsRef<str>) -> Result<()> {
    let words = BufReader::new(std::fs::File::open(path.as_ref()).unwrap())
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();

    // create a new rca with u16 as pointers (this limit data to u16::MAX bytes max size)
    let mut rcab = <RearCodedListBuilder>::new(4);
    rcab.extend(words.iter());

    rcab.print_stats();
    let rca = rcab.build();

    assert_eq!(rca.len(), words.len());

    // test that we can decode every string
    for (i, word) in words.iter().enumerate() {
        assert_eq!(&rca.get(i), word);
    }

    // test that the iter is correct
    for (i, word) in rca.into_iter().enumerate() {
        assert_eq!(word, words[i]);
    }

    for from in 0..rca.len() {
        for (i, word) in rca.into_iter_from(from).enumerate() {
            assert_eq!(word, words[i + from]);
        }
    }

    assert!(!rca.contains(""));

    for word in words.iter() {
        assert!(rca.contains(word));
        let mut word = word.clone();
        word.push_str("IT'S HIGHLY IMPROBABLE THAT THIS STRING IS IN THE WORDLIST");
        assert!(!rca.contains(&word));
    }

    let tmp_file = std::env::temp_dir().join("test_serdes_rcl.bin");
    let mut file = std::io::BufWriter::new(std::fs::File::create(&tmp_file)?);
    let schema = rca.serialize_with_schema(&mut file)?;
    drop(file);
    println!("{}", schema.to_csv());

    let c = <RearCodedList>::mmap(&tmp_file, epserde::deser::Flags::empty())?;

    for (i, word) in words.iter().enumerate() {
        assert_eq!(&c.get(i), word);
    }

    Ok(())
}
