use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

struct Cpu {
    ram: [u8; 4096],
    v: [u8; 16],
    i_reg: u16,
    pc: u16,
    sp: u8,
    stack: [u16; 16],
    sound: u8,
    delay: u8
}

fn opcode_not_implemented(opcode: u16) {
  print!("opcode not implemented: {:#x}", opcode);
  //EMBED_BREAKPOINT;
  process::exit(0);
}

/* BEGINNING OF OPCODES */
//00E0 - CLS
//Clear the display.
fn cls() {
  //Don't do anything since I don't have a rendering method yet
}

//00EE - RET
//Return from a subroutine.
fn ret(cpu: &mut Cpu) {
  cpu.sp -= 1;
  cpu.pc = cpu.stack[cpu.sp as usize];
}

//0nnn - SYS addr
//Jump to a machine code routine at nnn.
fn sys() {
  //Do nothing
}

//1nnn - JP addr
//Jump to location nnn.
fn jp_addr(cpu: &mut Cpu, opcode: u16) {
  let bcd = opcode & 0x0FFF;
  cpu.pc = bcd;
}


//2nnn - CALL addr
//Call subroutine at nnn.
fn call(cpu: &mut Cpu, opcode: u16) {
  //EMBED_BREAKPOINT;
  let bcd = opcode & 0x0FFF;
  cpu.sp += 1;
  cpu.pc = bcd;
}

//3xkk - SE Vx, byte
//Skip next instruction if Vx = kk.
fn se_vx_byte(cpu: &mut Cpu, opcode: u16) {
  let b = (opcode & 0x0F00) >> 8;
  let cd = (opcode & 0x00FF) as u8;
  if cpu.v[b as usize] == cd {
    cpu.pc += 2;
  }
}

//4xkk - SNE Vx, byte
//Skip next instruction if Vx != kk.
fn sne_vx_byte(cpu: &mut Cpu, opcode: u16) {
  let b = (opcode & 0x0F00) >> 8;
  let cd = (opcode & 0x00FF) as u8;
  if cpu.v[b as usize] != cd {
    cpu.pc += 2;
  }
}

fn se_vx_vy(cpu: &mut Cpu, opcode: u16) {
  opcode_not_implemented(opcode);
}

//6xkk - LD Vx, byte
//The interpreter puts the value kk into register Vx.
fn ld_vx_byte(cpu: &mut Cpu, opcode: u16) {
  let b = (opcode & 0x0F00) >> 8;
  let cd = (opcode & 0x00FF) as u8;
  cpu.v[b as usize] = cd;
}

//7xkk - ADD Vx, byte
//Set Vx = Vx + kk.
fn add_vx_byte(cpu: &mut Cpu, opcode: u16) {
  let b = (opcode & 0x0F00) >> 8;
  let cd = (opcode & 0x00FF) as u8;
  cpu.v[b as usize] += cd;
}

fn ld_vx_vy(cpu: &mut Cpu, opcode: u16) {
  opcode_not_implemented(opcode);
}

fn or_vx_vy(cpu: &mut Cpu, opcode: u16) {
  opcode_not_implemented(opcode);
}

//8xy2 - AND Vx, Vy
//Set Vx = Vx AND Vy.
fn and_vx_vy(cpu: &mut Cpu, opcode: u16) {
  let b = (opcode & 0x0F00) >> 8;
  let c = (opcode & 0x00F0) >> 4;
  cpu.v[b as usize] = cpu.v[b as usize] & cpu.v[c as usize];
}

fn xor_vx_vy(cpu: &mut Cpu, opcode: u16) {
  opcode_not_implemented(opcode);
}

fn add_vx_vy(cpu: &mut Cpu, opcode: u16) {
  opcode_not_implemented(opcode);
}

fn sub_vx_vy(cpu: &mut Cpu, opcode: u16) {
  opcode_not_implemented(opcode);
}

fn shr_vx_vy(cpu: &mut Cpu, opcode: u16) {
  opcode_not_implemented(opcode);
}

fn subn_vx_vy(cpu: &mut Cpu, opcode: u16) {
  opcode_not_implemented(opcode);
}

fn shl_vx_vy(cpu: &mut Cpu, opcode: u16) {
  opcode_not_implemented(opcode);
}

fn sne_vx_vy(cpu: &mut Cpu, opcode: u16) {
  opcode_not_implemented(opcode);
}

//Annn - LD I, addr
//Set I = nnn.
fn ld_i_addr(cpu: &mut Cpu, opcode: u16) {
  let bcd = opcode & 0x0FFF;
  cpu.i_reg = bcd;
}

fn jp_v0_addr(cpu: &mut Cpu, opcode: u16) {
  opcode_not_implemented(opcode);
}

fn rnd(cpu: &mut Cpu, opcode: u16) {
  opcode_not_implemented(opcode);
}

fn drw(cpu: &mut Cpu, opcode: u16) {
  opcode_not_implemented(opcode);
}

fn skp_vx(cpu: &mut Cpu, opcode: u16) {
  opcode_not_implemented(opcode);
}

fn sknp_vx(cpu: &mut Cpu, opcode: u16) {
  opcode_not_implemented(opcode);
}

fn ld_vx_dt(cpu: &mut Cpu, opcode: u16) {
  opcode_not_implemented(opcode);
}

fn ld_vx_k(cpu: &mut Cpu, opcode: u16) {
  opcode_not_implemented(opcode);
}

fn ld_dt_vx(cpu: &mut Cpu, opcode: u16) {
  opcode_not_implemented(opcode);
}

fn ld_st_vx(cpu: &mut Cpu, opcode: u16) {
  opcode_not_implemented(opcode);
}

//Fx1E - ADD I, Vx
//Set I = I + Vx.
fn add_i_vx(cpu: &mut Cpu, opcode: u16) {
  let b = opcode & 0x0F00 >> 8;
  cpu.i_reg += cpu.v[b as usize] as u16;
}

fn ld_f_vx(cpu: &mut Cpu, opcode: u16) {
  opcode_not_implemented(opcode);
}

fn ld_b_vx(cpu: &mut Cpu, opcode: u16) {
  opcode_not_implemented(opcode);
}

//Fx55 - LD [I], Vx
//Store registers V0 through Vx in memory starting at location I.
fn ld_iref_vx(cpu: &mut Cpu, opcode: u16) {
  let b = (opcode & 0x0F00) >> 8;
  for i in 0..b+1 {
    cpu.i_reg = i;
    cpu.ram[cpu.i_reg as usize] = cpu.v[i as usize];
  }
  cpu.i_reg += 1;
}

//Fx65 - LD Vx, [I]
//Read registers V0 through Vx from memory starting at location I.
fn ld_vx_iref(cpu: &mut Cpu, opcode: u16) {
  let b = (opcode & 0x0F00) >> 8;
  for i in 0..b+1 {
    cpu.i_reg = i;
    cpu.v[i as usize] = cpu.ram[cpu.i_reg as usize];
  }
  cpu.i_reg += 1;
}
/* END OF OPCODES */

/*
*/

fn next_opcode(cpu: &mut Cpu) {
  let mut opcode: u16;
  opcode = ((cpu.ram[cpu.pc as usize]) as u16) << 8;
  opcode = opcode | (cpu.ram[(cpu.pc + 1) as usize]) as u16;
  print!("current opcode: {:#x}", opcode);
  cpu.pc += 2;
  cpu.stack[cpu.sp as usize] = cpu.pc;
       if opcode == 0x00E0 { cls(); }
  else if opcode == 0x00EE { ret(cpu); }
  else if opcode <  0x1000 { sys(); }
  else if opcode <  0x2000 { jp_addr(cpu, opcode); }
  else if opcode <  0x3000 { call(cpu, opcode); }
  else if opcode <  0x4000 { se_vx_byte(cpu, opcode); }
  else if opcode <  0x5000 { sne_vx_byte(cpu, opcode); }
  else if opcode <  0x6000 { se_vx_vy(cpu, opcode); }
  else if opcode <  0x7000 { ld_vx_byte(cpu, opcode); }
  else if opcode <  0x8000 { add_vx_byte(cpu, opcode); }
  else if opcode <  0x9000 { 
    let d = opcode & 0x000F;
         if d == 0x0000 { ld_vx_vy(cpu, opcode); }
    else if d == 0x0001 { or_vx_vy(cpu, opcode); }
    else if d == 0x0002 { and_vx_vy(cpu, opcode); }
    else if d == 0x0003 { xor_vx_vy(cpu, opcode); }
    else if d == 0x0004 { add_vx_vy(cpu, opcode); }
    else if d == 0x0005 { sub_vx_vy(cpu, opcode); }
    else if d == 0x0006 { shr_vx_vy(cpu, opcode); }
    else if d == 0x0007 { subn_vx_vy(cpu, opcode); }
    else if d == 0x000E { shl_vx_vy(cpu, opcode); }
  }
  else if opcode <  0xA000 { sne_vx_vy(cpu, opcode); }
  else if opcode <  0xB000 { ld_i_addr(cpu, opcode); }
  else if opcode <  0xC000 { jp_v0_addr(cpu, opcode); }
  else if opcode <  0xD000 { rnd(cpu, opcode); }
  else if opcode <  0xE000 { drw(cpu, opcode); }
  else if opcode <  0xF000 {
    let cd = (opcode & 0x00FF) as u8;
         if cd == 0x009E { skp_vx(cpu, opcode); }
    else if cd == 0x00A1 { sknp_vx(cpu, opcode); }
  }
  else { 
    let cd = (opcode & 0x00FF) as u8;
         if cd == 0x0007 { ld_vx_dt(cpu, opcode); }
    else if cd == 0x000A { ld_vx_k(cpu, opcode); }
    else if cd == 0x0015 { ld_dt_vx(cpu, opcode); }
    else if cd == 0x0018 { ld_st_vx(cpu, opcode); }
    else if cd == 0x001E { add_i_vx(cpu, opcode); }
    else if cd == 0x0029 { ld_f_vx(cpu, opcode); }
    else if cd == 0x0033 { ld_b_vx(cpu, opcode); }
    else if cd == 0x0055 { ld_iref_vx(cpu, opcode); }
    else if cd == 0x0065 { ld_vx_iref(cpu, opcode); }
  }
}

fn create_cpu() -> Cpu {
  return Cpu {
    ram: [0; 4096],
    v: [0; 16],
    i_reg: 0,
    pc: 0,
    sp: 0,
    stack: [0; 16],
    sound: 0,
    delay: 0
  };
}

fn file_to_mem(cpu: &mut Cpu, file: String) {
    let mut file = File::open(file).expect("Unable to open the file");
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).expect("Unable to read the file");
    let mut index = 0;
    for byte in contents.iter() {
      cpu.ram[index] = *byte;
      index += 1;
    }
}

fn main() {
  let mut cpu = create_cpu();
  let mut arguments = env::args();
  let file = arguments.nth(1).expect("No first argument provided");
  file_to_mem(&mut cpu, file);
  loop {
    next_opcode(&mut cpu);
  }
}
 