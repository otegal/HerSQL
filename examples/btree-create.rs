use anyhow::Result;

use HerSQL::disk::DiskManager;
use HerSQL::buffer::{BufferPool, BufferPoolManager};
use HerSQL::btree::BTree;

fn main() -> Result<()> {
    let disk = DiskManager::open("test.btr")?;
    let pool = BufferPool::new(10);
    let mut bufmgr = BufferPoolManager::new(disk, pool);
    let btree = BTree::create(&mut bufmgr)?;

    btree.insert(&mut bufmgr, b"Kanagawa", b"Yokohama");
    btree.insert(&mut bufmgr, b"Osaka", b"Osaka");
    btree.insert(&mut bufmgr, b"Aichi", b"Nagoya");
    btree.insert(&mut bufmgr, b"Hokkaido", b"Sapporo");
    btree.insert(&mut bufmgr, b"Fukuoka", b"Fukuoka");
    btree.insert(&mut bufmgr, b"Hyogo", b"Kobe");

    bufmgr.flush()?;

    Ok(())
}