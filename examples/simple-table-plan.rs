use anyhow::Result;

use HerSQL::query::{Filter, SeqScan, TupleSearchMode, PlanNode};
use HerSQL::buffer::{BufferPool, BufferPoolManager};
use HerSQL::disk::{DiskManager, PageId};
use HerSQL::tuple;

fn main() -> Result<()> {
    let disk = DiskManager::open("simple.her")?;
    let pool = BufferPool::new(10);
    let mut bufmgr = BufferPoolManager::new(disk, pool);

    let plan = Filter {
        cond: &|record| record[1].as_slice() < b"Dave",
        inner_plan: &SeqScan {
            table_meta_page_id: PageId(0),
            search_mode: TupleSearchMode::Key(&[b"w"]),
            while_cond: &|pkey| pkey[0].as_slice() < b"z",
        },
    };
    let mut exec = plan.start(&mut bufmgr)?;

    while let Some(record) = exec.next(&mut bufmgr)? {
        println!("{:?}", tuple::Pretty(&record));
    }
    Ok(())
}