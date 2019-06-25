//!
//! https://tools.ietf.org/html/rfc5161
//!
//! IMAP ENABLE extension
//!

#![cfg_attr(rustfmt, rustfmt_skip)]
#![cfg_attr(feature = "cargo-clippy", allow(redundant_closure))]


use nom::IResult;

use std::str;
use core::*;
use types::*;



named!(pub enabled<Response>, do_parse!(
        tag_s!("ENABLED") >>
        capabilities: map!(many0!(preceded!(char!(' '), map!(atom, |a| Capability::Atom(a)))), |c| Response::Capabilities(c)) >>
        (capabilities)
    ));
        // capabilities: Response::Capabilities(cap_vec) >>

pub fn test_enabled()
{
    let r = enabled(b"ENABLED COI MOI NOI\r\n");
    // let r = astring_utf8(b"ENABLED ");
    // let r = tst_astring(b"ENABLED");
    // let r = separated_list_complete!(b"ENABLED COI MOI NOI", tag!(" "), tag!("ENABLED"));
    // // let r = string_utf8(b"ENABLED COI MOI NOI");
    println!("{:?}", r);
}
