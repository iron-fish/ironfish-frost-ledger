/*****************************************************************************
 *   Ledger App Boilerplate Rust.
 *   (c) 2023 Ledger SAS.
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 *****************************************************************************/

use core::default;

use crate::app_ui::secret::ui_display_secret;
use crate::utils::Bip32Path;
use crate::AppSW;
use crate::Instruction;
use ironfish_frost::{dkg::round1::{self, PublicPackage}, participant::{Secret, SECRET_LEN,}};

extern crate alloc;
use alloc::vec::Vec;

use ledger_device_sdk::{ecc::{Secp256k1, SeedDerive}, io::ApduHeader};
use ledger_device_sdk::random::LedgerRng;
use ledger_device_sdk::hash::{sha3::Keccak256, HashInit};
use ledger_device_sdk::io::{Comm, Event};

const MAX_APDU_SIZE: usize = 255;

fn send_apdu_chunks(comm: &mut Comm, data: &[u8]) -> Result<(), AppSW> {
    let total_size = data.len();
    let mut offset = 0;

    while offset < total_size {
        let end = usize::min(offset + MAX_APDU_SIZE, total_size);
        let chunk = &data[offset..end];
        comm.append(chunk);

        // Send the chunk (you may need to handle the sending mechanism depending on your Comm implementation)
        comm.reply_ok();
        match comm.next_event() {
            Event::Command(Instruction::GenerateSecret { display }) => {}
            _ => return Err(AppSW::ClaNotSupported),
        }
        
        offset = end;
    }

    Ok(())
}

pub fn handler_generate_secret(comm: &mut Comm, display: bool) -> Result<(), AppSW> {
    let mut rng = LedgerRng {};

    let secret1 = Secret::random(&mut rng);
    let identity1 = secret1.to_identity();
    let secret2 = Secret::random(&mut rng);
    let identity2 = secret2.to_identity();

    let (_round1_secret_package, package): (Vec<u8>, PublicPackage) = round1::round1(
        &identity1,
        2,
        [&identity1, &identity2],
        &mut rng,
    ).unwrap();

    let round1_secret_slice = &package.serialize()[..];
    if display {
        if !ui_display_secret(round1_secret_slice)? {
            return Err(AppSW::Deny);
        }
    }

    // Send the data in chunks
    send_apdu_chunks(comm, round1_secret_slice)?;

    Ok(())
}