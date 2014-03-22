
#[allow(deprecated_owned_vector)];

use packetdissector::{IpHeader, TcpHeader};
use std::iter;
use std::mem::size_of_val;
use std::mem::to_be16;
use std::slice;

pub fn ip_header(iphdr: &mut IpHeader) {
    let iphdr_len = size_of_val(iphdr);
    let iphdr_ptr: *u8 = iphdr as *mut IpHeader as *u8;
    let iphdr_v = unsafe { slice::from_buf(iphdr_ptr, iphdr_len) };
    let mut sum: u64 = iter::range_step(0, iphdr_len as u64, 2).
        fold(0u64, |sum, i|
             sum + ((iphdr_v[i] as u16 << 8) | iphdr_v[i + 1] as u16) as u64);
    while (sum >> 16) != 0 {
        sum = (sum & 0xffff) + (sum >> 16);
    }
    let iphdr: &mut IpHeader = unsafe { &mut *(iphdr_ptr as *mut IpHeader) };
    iphdr.ip_sum = to_be16(!(sum as u16) as i16) as u16;
}

pub fn tcp_header(iphdr: &IpHeader, tcphdr: &mut TcpHeader) {
    let tcphdr_len = ((tcphdr.th_off_x2 >> 4) & 0xf) as uint * 4;
    assert!(tcphdr_len >= size_of_val(tcphdr));
    let mut sum0: u64;
    sum0  = tcphdr_len as u64;
    sum0 += iphdr.ip_p as u64;
    sum0 += (iphdr.ip_src[0] as u16 << 8 | iphdr.ip_src[1] as u16) as u64;
    sum0 += (iphdr.ip_src[2] as u16 << 8 | iphdr.ip_src[3] as u16) as u64;
    sum0 += (iphdr.ip_dst[0] as u16 << 8 | iphdr.ip_dst[1] as u16) as u64;
    sum0 += (iphdr.ip_dst[2] as u16 << 8 | iphdr.ip_dst[3] as u16) as u64;
    let tcphdr_ptr: *u8 = tcphdr as *mut TcpHeader as *u8;
    let tcphdr_v = unsafe { slice::from_buf(tcphdr_ptr, tcphdr_len) };
    let mut sum: u64 = iter::range_step(0, tcphdr_len as u64, 2).
        fold(sum0, |sum, i|
             sum + ((tcphdr_v[i] as u16 << 8) | tcphdr_v[i + 1] as u16) as u64);
    while (sum >> 16) != 0 {
        sum = (sum & 0xffff) + (sum >> 16);
    }
    let tcphdr: &mut TcpHeader = unsafe { &mut *(tcphdr_ptr as *mut TcpHeader) };
    tcphdr.th_sum = to_be16(!(sum as u16) as i16) as u16;
}
