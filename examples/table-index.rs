
use anyhow::Result;

use HerSQL::query::{IndexScan, TupleSearchMode, PlanNode};
use HerSQL::buffer::{BufferPool, BufferPoolManager};
use HerSQL::disk::{DiskManager, PageId};
use HerSQL::tuple;

// SELECT * WHERE last_name = 'Smith'
// with index
fn main() -> Result<()> {
    let disk = DiskManager::open("table.her")?;
    let pool = BufferPool::new(10);
    let mut bufmgr = BufferPoolManager::new(disk, pool);

    let plan = IndexScan {
        table_meta_page_id: PageId(0),
        index_meta_page_id: PageId(2),
        search_mode: TupleSearchMode::Key(&[b"Smith"]),
        while_cond: &|skey| skey[0].as_slice() == b"Smith",
    };
    let mut exec = plan.start(&mut bufmgr)?;

    while let Some(record) = exec.next(&mut bufmgr)? {
        println!("{:?}", tuple::Pretty(&record));
    }
    Ok(())
}