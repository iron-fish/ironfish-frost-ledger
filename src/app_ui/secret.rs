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

 use crate::AppSW;
 use alloc::format;
 
 #[cfg(not(any(target_os = "stax", target_os = "flex")))]
 use ledger_device_sdk::ui::{
     bitmaps::{CROSSMARK, EYE, VALIDATE_14},
     gadgets::{Field, MultiFieldReview},
 };
 
 #[cfg(any(target_os = "stax", target_os = "flex"))]
 use ledger_device_sdk::nbgl::{NbglAddressReview, NbglGlyph};
 
 #[cfg(any(target_os = "stax", target_os = "flex"))]
 use include_gif::include_gif;
 
 // Display only the last 20 bytes of the address
 const DISPLAY_ADDR_BYTES_LEN: usize = 20;
 
 pub fn ui_display_secret(secret: &[u8]) -> Result<bool, AppSW> {
     let secret_hex = format!(
         "0x{}",
         hex::encode(&secret).to_uppercase()
     );
 
     #[cfg(not(any(target_os = "stax", target_os = "flex")))]
     {
         let my_field = [Field {
             name: "Frost Secret",
             value: secret_hex.as_str(),
         }];
 
         let my_review = MultiFieldReview::new(
             &my_field,
             &["Confirm Frost Secret"],
             Some(&EYE),
             "Approve",
             Some(&VALIDATE_14),
             "Reject",
             Some(&CROSSMARK),
         );
 
         Ok(my_review.show())
     }
 
     #[cfg(any(target_os = "stax", target_os = "flex"))]
     {
         // Load glyph from 64x64 4bpp gif file with include_gif macro. Creates an NBGL compatible glyph.
         const FERRIS: NbglGlyph = NbglGlyph::from_include(include_gif!("crab_64x64.gif", NBGL));
         // Display the address confirmation screen.
         Ok(NbglAddressReview::new()
             .glyph(&FERRIS)
             .verify_str("Verify CRAB address")
             .show(&addr_hex))
     }
 }
 