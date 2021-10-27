#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;

pub use self::privilege_manage::{
    PrivilegeManage,
};
#[ink::contract]
mod privilege_manage {
    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use ink_prelude::collections::BTreeMap;
    use ink_storage::{collections::HashMap as StorageHashMap, };


    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct PrivilegeManage {
        owner:AccountId,
        index:u64,
        privilege_map:StorageHashMap<u64,String>,

    }

    impl PrivilegeManage {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            let instance = Self {
                owner:Self::env().caller(),
                index: 0,
                privilege_map : StorageHashMap::new(),
            };
            instance
        }

        #[ink(message)]
        pub fn add_privilege(&mut self, name: String) -> bool {
            assert_eq!(self.index + 1 > self.index, true);
            self.privilege_map.insert(self.index, name);
            self.index += 1;
            true
        }

        #[ink(message)]
        pub fn list_privileges(&self) -> Vec<String> {
            let mut privilege_vec = Vec::new();
            let mut iter = self.privilege_map.values();
            let mut privilege = iter.next();
            while privilege.is_some() {
                privilege_vec.push(privilege.unwrap().clone());
                privilege = iter.next();
            }
            privilege_vec
        }

        #[ink(message)]
        pub fn query_privilege_by_index(&self, index: u64) -> String {
            self.privilege_map.get(&index).unwrap().clone()
        }

    }


    // #[cfg(test)]
    // mod tests {
    //     /// Imports all the definitions from the outer scope so we can use them here.
    //     use super::*;
    //
    //     /// Imports `ink_lang` so we can use `#[ink::test]`.
    //     use ink_lang as ink;
    //
    //     /// We test if the default constructor does its job.
    //     #[ink::test]
    //     fn default_works() {
    //         let privilegeManage = PrivilegeManage::default();
    //         assert_eq!(privilegeManage.get(), false);
    //     }
    //
    //     /// We test a simple use case of our contract.
    //     #[ink::test]
    //     fn it_works() {
    //         let mut privilegeManage = PrivilegeManage::new(false);
    //         assert_eq!(privilegeManage.get(), false);
    //         privilegeManage.flip();
    //         assert_eq!(privilegeManage.get(), true);
    //     }
    // }
}
