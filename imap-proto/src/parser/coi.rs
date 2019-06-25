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

named!(mailbox_root<&str>, do_parse!(
    tag_s!("MAILBOX-ROOT ") >>
        mbox: astring_utf8 >>
        (mbox)
));

named!(pub resp_coi<Response>, do_parse!(
    tag_s!("COI ") >>
        mbox:    map!(map!(mailbox_root, |mailbox| MailboxDatum::Status{mailbox, status:Vec::new()}), |mb| Response::MailboxData(mb)) >>
        // mbox:    map!(mailbox_root, |m| MailboxDatum::Status{m, std::vec::Vec<StatusAttribute>::new()}) >>
        (mbox)
));

pub fn test_mbox_root()
{
    let r = resp_coi(b"COI MAILBOX-ROOT COI\r\n");
    // let r = mailbox_root(b"MAILBOX-ROOT COI\r\n");
    // let r = astring_utf8(b"enabled ");
    // let r = tst_astring(b"ENABLED");
    // let r = separated_list_complete!(b"ENABLED COI MOI NOI", tag!(" "), tag!("ENABLED"));
    // // let r = string_utf8(b"ENABLED COI MOI NOI");
    println!("{:?}", r);
}
