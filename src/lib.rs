#[cfg(test)]
mod tests {

    use chrono::NaiveDate;
    use hex_literal::hex;
    use parity_scale_codec::Compact;
    use sp_runtime::testing::H256;
    use sp_runtime::traits::BlakeTwo256;
    use sp_trie::StorageProof;
    use sp_trie::Trie;
    use std::borrow::Borrow;

    //First make sure you start node from substrate node template.
    // Steps on making your substrate custom blockchain vistit -- substrate.io
    //Then connect your node to Polkadot Js

    #[test]
    fn it_works() {
        //query storage key from polkadot js
        let key = hex!("f0c365c3cf59d671eb72da0e7a4113c49f1f0515f462cdcf84e0f1d6045dfcbb").to_vec();

        //Using block hash get the state root
        let state_root = H256::from(hex!(
            "dd9eff6b4e58b35fd7954298749182d899cd48f35ba4a9e9b0fc0fafd14980fa"
        ));

        //Using block hash and storage key , head over Developer -> RPC queries -> choose State -> Then the method is getReadProof

        //These are trie Node, If you dont know them head over https://www.youtube.com/watch?v=lutdaQl6Mzo&list=PLLBglgxW6ibJ6tUDKEcPY0GUia4wgd-Sx&index=17

        let proof = vec![
            hex!("802e988035f7cde2e1a25c6eec689191e08e896725f959a7ef6b0cbb31f516ff48a4b1fa80830992d7516b674941a739c2b411dede725920bb0d4c7b4023dc1d13b4537bc6801241725a8fcbcef72f4aa6a35c15cacd0539b7b7a8c0f80c7f7b85614ff41651803d00f158a65cae6da0f9a8f17faba0d638318e5ec5113096fd1aec462b27788b80b910801c17628eb6be93d75082280e6e08c5ef4bd867874a7d2d94af6ec9ac0380ed02fa8bdf827f5fe07d59a52c99d5ac87e90c4234a222315b4d118ce84e7db480c3329a284d1a2645ea409b3df19c033b1742e626d8cfe93ad99045c4187ebf8b").to_vec(),
            hex!("9f00c365c3cf59d671eb72da0e7a4113c41002505f0e7b9012096b41c4eb3aaf947f6ea429080000685f0f1f0515f462cdcf84e0f1d6045dfcbb2091cfd98f80010000").to_vec()
        ];

        //Creating a db from those trie nodes
        let db = StorageProof::new(proof).into_memory_db();

        //Creating trie data structure perssistent DB
        let trie =
            sp_trie::TrieDB::<sp_trie::LayoutV0<BlakeTwo256>>::new(&db, &state_root).unwrap();

        let mut value = trie.get(&key).unwrap().unwrap();

        // The result is array of bytes (encoded data) So we need to Decode it to original type
        let timestamp: u64 = parity_scale_codec::Decode::decode(&mut &value[..]).unwrap();

        //At that particular block height the timestamp is indeed same, HENCE PROVED
        assert_eq!(1651680858001, timestamp)
    }
}
