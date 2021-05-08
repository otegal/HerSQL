use anyhow::Result;

use HerSQL::btree::{BTree, SearchMode};
use HerSQL::buffer::{BufferPool, BufferPoolManager};
use HerSQL::disk::{DiskManager, PageId};
use HerSQL::tuple;

fn main() -> Result<()> {
    let disk = DiskManager::open("simple.her")?;
    let pool = BufferPool::new(10);
    let mut bufmgr = BufferPoolManager::new(disk, pool);

    let btree = BTree::new(PageId(0));
    let mut iter = btree.search(&mut bufmgr, SearchMode::Start)?;

    // デコードして表示する
    while let Some((key, value)) = iter.next(&mut bufmgr)? {
        let mut record = vec![];
        tuple::decode(&key, &mut record);
        tuple::decode(&value, &mut record);
        println!("{:?}", tuple::Pretty(&record));
    }
    Ok(())
}