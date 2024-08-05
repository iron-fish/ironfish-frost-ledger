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

 use crate::app_ui::secret::ui_display_secret;
 use crate::utils::Bip32Path;
 use crate::AppSW;
 use ironfish_frost::participant::{Secret, SECRET_LEN};

 use ledger_device_sdk::ecc::{Secp256k1, SeedDerive};
 use ledger_device_sdk::random::LedgerRng;
 use ledger_device_sdk::hash::{sha3::Keccak256, HashInit};
 use ledger_device_sdk::io::Comm;
 
 pub fn handler_generate_secret(comm: &mut Comm, display: bool) -> Result<(), AppSW> {
    //  let data = comm.get_data().map_err(|_| AppSW::WrongApduLength)?;
    //  let path: Bip32Path = data.try_into()?;
    //  let cc = data.get_opt(0)?;
 
     let mut rng = LedgerRng {};
    
    let secret = Secret::random(&mut rng);
    let secret_bytes = secret.serialize();
 
     if display {
         if !ui_display_secret(&secret_bytes)? {
             return Err(AppSW::Deny);
         }
     }
 
     comm.append(&[SECRET_LEN as u8]);
     comm.append(&secret_bytes);
 
     Ok(())
 }
 