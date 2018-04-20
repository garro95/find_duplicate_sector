/*
 *  Copyright © 2018 Gianmarco Garrisi
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */
#[macro_use] extern crate quicli;

use quicli::prelude::*;

use std::{fs::File, io::{Read, BufReader, SeekFrom, prelude::*}};

/// Find the first duplicate of a sector on a partition.
///
/// This program is free software: it is distributed under the terms
/// of the GNU General Public License as published by the
/// Free Software Foundation, either version 3 of the License, or
/// (at your option) any later version.
#[derive(Debug, StructOpt)]
struct Interface {
    /// The path to the partition to analyze e.g. /dev/sda1
    partition_name: String,
    /// The absolute offset of the sector to look for
    ref_sector: u64,
    /// The absolute offset of the sector where to start the search
    start_sector:u64,
    /// The absolute offset of the sector where to end the search
    end_sector: u64,
    /// Level of verbosity
    #[structopt(long = "verbose", short = "v", parse(from_occurrences))]
    verbosity: u8,
    #[structopt(long="bytes", short="b", default_value="512")]
    bytes_per_sector: u64,
}

main!( |args: Interface, log_level: verbosity| {
    assert!(args.start_sector < args.end_sector, "start_sector is greater then end_sector");

    let mut partition = File::open(args.partition_name)?;
    let mut ref_sector = vec![0;args.bytes_per_sector as usize];

    partition.seek(SeekFrom::Start(args.ref_sector*args.bytes_per_sector))?;
    partition.read_exact(&mut ref_sector)?;

    partition.seek(SeekFrom::Start(args.start_sector))?;
    let mut partition = BufReader::new(partition);

    let mut i:u64 = args.start_sector;
    let mut buf = vec![0;args.bytes_per_sector as usize];
    while i <= args.end_sector && buf != ref_sector {
        partition.read_exact(&mut buf)?;
        i+=1;
        if ((i*args.bytes_per_sector) | 0b11_1111_1111_1111_1111_1111_1111_1111 /* 1Gib-1 */ ) == 0  {
            info!("Sector {} ({} GiB, {} GB)", i, (i*args.bytes_per_sector)>>30, i*args.bytes_per_sector/1_000_000_000)
        }
    }

    if i > args.end_sector {
        println!("Duplicate sector not found");
    } else {
        println!("{} is the first duplicate sector", i);
    }
});
