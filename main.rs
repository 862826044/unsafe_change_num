use geiger::find_unsafe_in_string;
use geiger::RsFileMetrics;
use geiger::IncludeTests;
use geiger::find_unsafe_in_file;
use std::path::Path;
use walkdir::WalkDir;
use std::fs::read_dir;
use cargo_geiger_serde::CounterBlock;
use cargo_geiger_serde::Count;
use std::ops::Add;



fn main() {
    let includetests = IncludeTests :: No;//不包含测试代码块
    //let file = std::fs::File::open("F:\\Python\\result_1\\1_mystor_rust-cpp_commit_003d0984bab27726a629122d66c88c6738f0fd02_0.rs").unwrap();
    //println!("文件打开成功：{:?}",file);
    //let rsfile = find_unsafe_in_file(Path::new("F:\\Python\\result_1\\1_mystor_rust-cpp_commit_003d0984bab27726a629122d66c88c6738f0fd02_0.rs"), includetests);
    //得到结果目录下的所有文件，并进行解析
/*     match rsfile
    {
        Err(e) =>
        {
            println!("Failed to parse file: {}", e);
        }
        Ok(scan_result) =>
        {
            println!("{:?}", scan_result);
        }
    } */
#[derive(Debug,Default)]
struct UNSAFE_CHANGE
{
    functions: i32,
    exprs: i32,
    item_impls: i32,
    item_traits: i32,
    methods: i32,
}
impl Add for UNSAFE_CHANGE {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            functions: self.functions + other.functions,
            exprs: self.exprs + other.exprs,
            item_impls: self.item_impls + other.item_impls,
            item_traits: self.item_traits + other.item_traits,
            methods: self.methods + other.methods,
        }
    }
}
#[derive(Debug,Default)]
struct _CHANGE
{
    to_unsafe: i32,
    to_safe: i32,
}
#[derive(Debug,Default)]
struct UNSAFE_CHANGE_PLUS
{
    functions: _CHANGE,
    exprs: _CHANGE,
    impls: _CHANGE,
    traits: _CHANGE,
    methods: _CHANGE,
}

    //使用std::fs::read_dir
    let paths = "F:\\Python\\result_test";
    let path = Path::new(paths);

    let readdir = read_dir(path) . unwrap();

    let mut parse_fail_num = 0;
    let mut parse_success_num = 0;
    let mut open_fail = 0;
    let mut all_num = 0;
    let mut counters_0 = CounterBlock 
        { 
            functions: 
                Count { safe: 0, unsafe_: 0 }, 
            exprs: 
                Count { safe: 0, unsafe_: 0 }, 
            item_impls: 
                Count { safe: 0, unsafe_: 0 }, 
            item_traits: 
                Count { safe: 0, unsafe_: 0 }, 
            methods: 
                Count { safe: 0, unsafe_: 0 } 
        }; 
    let mut counters_1 = CounterBlock 
        { 
            functions: 
                Count { safe: 0, unsafe_: 0 }, 
            exprs: 
                Count { safe: 0, unsafe_: 0 }, 
            item_impls: 
                Count { safe: 0, unsafe_: 0 }, 
            item_traits: 
                Count { safe: 0, unsafe_: 0 }, 
            methods: 
                Count { safe: 0, unsafe_: 0 } 
        }; 
    let mut unsafe_change = UNSAFE_CHANGE
        {
            functions: 0,
            exprs: 0,
            item_impls: 0,
            item_traits: 0,
            methods: 0
        };

    
    let mut temp:UNSAFE_CHANGE = Default::default();
    //标识0,1
    let mut flag = 0;

    let mut unsafe_change_plus:UNSAFE_CHANGE_PLUS = Default::default();
    
    let mut temp_counterblock:CounterBlock = Default::default();
    for item in readdir
    {
        match item
        {
            Ok (entry) => {
                //println!("entry = {:?}",entry);
                //let path0 = entry.path().as_path();
                
                let rsfile = find_unsafe_in_file(entry.path().as_path(), includetests);
                match rsfile
                {
                    Err(e) =>
                    {
                        //println!("Failed to parse file: {}", e);
                        parse_fail_num = parse_fail_num + 1;
                    }
                    Ok(scan_result) =>
                    {
                        println!("{} = {:?}",parse_success_num,entry.path());
                        //println!("{:?}", scan_result);
                        if flag == 0
                        {
                            //counters_0 = counters_0 + scan_result.counters;
                            temp_counterblock = scan_result.counters;
                            flag = 1;
                            //let a = scan_result.counters.functions.unsafe_ + 0;
                            //temp.functions = scan_result.counters.functions.unsafe_ as i32;
                            //temp.exprs = scan_result.counters.exprs.unsafe_ as i32;
                            //temp.item_impls = scan_result.counters.item_impls.unsafe_ as i32;
                            //temp.item_traits = scan_result.counters.item_traits.unsafe_ as i32;
                            //temp.methods = scan_result.counters.methods.unsafe_ as i32;

                        }
                        else
                        {
                            //unsafe_change.functions = unsafe_change.functions + (temp.functions - scan_result.counters.functions.unsafe_ as i32).abs();
                            //unsafe_change.exprs = unsafe_change.exprs + (temp.exprs - scan_result.counters.exprs.unsafe_ as i32).abs();
                            //unsafe_change.item_impls = unsafe_change.item_impls + (temp.item_impls - scan_result.counters.item_impls.unsafe_ as i32).abs();
                            //unsafe_change.item_traits = unsafe_change.item_traits + (temp.item_traits - scan_result.counters.item_traits.unsafe_ as i32).abs();
                            //unsafe_change.methods = unsafe_change.methods + (temp.methods - scan_result.counters.methods.unsafe_ as i32).abs();

                            //let  unsafe_s = unsafe_change.add(temp);
                            //counters_1 = counters_1 + scan_result.counters;
                            flag = 0;
                            unsafe_change_plus.functions.to_safe = unsafe_change_plus.functions.to_safe + (temp_counterblock.functions.safe as i32- scan_result.counters.functions.safe as i32).abs();
                            unsafe_change_plus.functions.to_unsafe = unsafe_change_plus.functions.to_unsafe + (temp_counterblock.functions.unsafe_ as i32- scan_result.counters.functions.unsafe_ as i32).abs();
                            
                            unsafe_change_plus.exprs.to_safe = unsafe_change_plus.exprs.to_safe + (temp_counterblock.exprs.safe as i32- scan_result.counters.exprs.safe as i32).abs();
                            unsafe_change_plus.exprs.to_unsafe = unsafe_change_plus.exprs.to_unsafe + (temp_counterblock.exprs.unsafe_ as i32- scan_result.counters.exprs.unsafe_ as i32).abs();
                            
                            unsafe_change_plus.impls.to_safe = unsafe_change_plus.impls.to_safe + (temp_counterblock.item_impls.safe as i32- scan_result.counters.item_impls.safe as i32).abs();
                            unsafe_change_plus.impls.to_unsafe = unsafe_change_plus.impls.to_unsafe + (temp_counterblock.item_impls.unsafe_ as i32- scan_result.counters.item_impls.unsafe_ as i32).abs();
                            
                            unsafe_change_plus.methods.to_safe = unsafe_change_plus.methods.to_safe + (temp_counterblock.methods.safe as i32- scan_result.counters.methods.safe as i32).abs();
                            unsafe_change_plus.methods.to_unsafe = unsafe_change_plus.methods.to_unsafe + (temp_counterblock.methods.unsafe_ as i32- scan_result.counters.methods.unsafe_ as i32).abs();
                            
                            unsafe_change_plus.traits.to_safe = unsafe_change_plus.traits.to_safe + (temp_counterblock.item_traits.safe as i32- scan_result.counters.item_traits.safe as i32).abs();
                            unsafe_change_plus.traits.to_unsafe = unsafe_change_plus.traits.to_unsafe + (temp_counterblock.item_traits.unsafe_ as i32- scan_result.counters.item_traits.unsafe_ as i32).abs();
                        }
                        //counters = counters + scan_result.counters;
                        parse_success_num = parse_success_num + 1;
                    }
                }

            }
            Err(e) => {
                println!("Error");
                open_fail = open_fail + 1;
                
            }
            
        }

        //println!("{:?}",item);
    }
    all_num = open_fail + parse_success_num + parse_fail_num;


    let unsafe_ : UNSAFE_CHANGE = Default::default();
    println!("all_num = {}",all_num);
    println!("open_fail = {}",open_fail);
    println!("parse_fail_num = {}",parse_fail_num);
    println!("parse_success_num = {}",parse_success_num);
    
    //-----1
    //println!("修改前的unsafe使用情况\n {:?}",counters_0);
    //println!("修改后的unsafe使用情况\n {:?}",counters_1);
    
    //-----2
    //println!("unsafe的变动情况\n {:?}",unsafe_change);
    
    //-----3
    println!("unsafe的变动情况\n {:?}",unsafe_change_plus);
    
    
    
    
    
    
    
    
    
    
    
    //使用walk_dir
/*     for entry in WalkDir::new(path)

    {
        let entry = entry.unwrap();
        //println!("{}",entry.path().display());
        let rsfile = find_unsafe_in_file(path, includetests);
        match rsfile
        {
            Err(e) =>
            {
                println!("Failed to parse file: {}", e);
            }
            Ok(scan_result) =>
            {
                println!("{:?}", scan_result);
            }
        }
    } */
}
