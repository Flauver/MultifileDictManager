use std::{collections::HashMap, fs};

fn main() {
    let mut 所有文件名 = Vec::new();
    for entry in fs::read_dir("data").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            所有文件名.push(path.to_str().unwrap()[5..].to_string())
        }
    }
    所有文件名.sort();
    let mut 码表 = HashMap::new();
    for 文件名 in 所有文件名 {
        let mut lines = Vec::new();
        for line in fs::read_to_string("data/".to_owned() + &文件名).unwrap().lines() {
            lines.push(line.split('\t')
                .map(|x| x.to_string())
                .collect::<Vec<_>>());
        }
        for entry in lines.iter().filter(|x| x.len() == 2 || ( x.len() == 3 && x[2].parse::<usize>().is_ok() )) {
            if entry.len() == 2 {
                if let Some(候选) = 码表.entry(entry[1].clone())
                    .or_insert(Vec::<Option<String>>::new())
                    .iter_mut()
                    .find(|x| x.is_none()) {
                    *候选 = Some(entry[0].clone());
                } else {
                    码表.get_mut(&entry[1]).unwrap().push(Some(entry[0].clone()));
                }
            } else {
                let 词 = entry[0].clone();
                let 码 = entry[1].clone();
                if let Ok(候选位) = entry[2].parse::<usize>() {
                    let 候选 = 码表.entry(码.clone()).or_default();
                    while 候选位 > 候选.len() {
                        候选.push(None);
                    }
                    码表.get_mut(&码).unwrap()[候选位 - 1] = Some(词);
                }
            }
        }
        for entry in lines.iter().filter(|x| x.len() == 4 || (x.len() == 3 && x[2].parse::<usize>().is_err())) {
            if entry.len() == 4 {
                let 词 = entry[0].clone();
                let 码 = entry[1].clone();
                let 候选位 = entry[2].parse::<usize>().unwrap();
                let mut 新候选 = 码表[&码].clone()
                    .into_iter()
                    .filter(|x| *x != Some(词.clone()))
                    .collect::<Vec<_>>();
                while 候选位 - 1 > 新候选.len() {
                    新候选.push(None);
                }
                新候选.insert(候选位 - 1, Some(词));
                码表.insert(码, 新候选);
            } else {
                if entry[2] == "删" {
                    码表.get_mut(&entry[1]).unwrap().retain(|x| *x != Some(entry[0].clone()));
                } else {
                    码表.insert(entry[0].clone(), entry[1].split(" ").map(|x| Some(x.to_string())).collect::<Vec<_>>());
                }
            }
        }
    }

    let mut 码表 = 码表.into_iter().collect::<Vec<_>>();
    码表.sort();
    码表.retain(|x| !x.1.is_empty() && !x.1.iter().all(|x| x.is_none()));
    let 码表 = 码表.into_iter().map(|(码, 词)| {
        let mut 候选 = Vec::new();
        for (i, 词) in 词.into_iter().enumerate() {
            if let Some(词) = 词 {
                候选.push(词);
            } else {
                println!("已自动补充编码 {} 的第 {} 项的占位文字", 码, i + 1);
                候选.push(format!("候选{}", i + 1));
            }
        }
        (码, 候选)
    }).collect::<Vec<_>>();

    println!("请选择你要导出的码表格式：");
    println!("1.多多格式（词tab码）");
    println!("2.单行多义格式（码tab词1 词2 词n）");
    println!("3.QQ格式（码=候选数,词）");

    let mut mode = String::new();
    let _ = std::io::stdin().read_line(&mut mode);

    let mut output = Vec::new();
    match mode.trim() {
        "1" => {
            for (码, 词) in 码表.iter() {
                for 词 in 词.iter() {
                    output.push(format!("{}\t{}", 词, 码));
                }
            }
        },

        "2" => {
            for (码, 词) in 码表.iter() {
                output.push(format!("{}\t{}", 码, 词.join(" ")));
            }
        },

        "3" => {
            for (码, 词) in 码表.iter() {
                for (i, 词) in 词.iter().enumerate() {
                    output.push(format!("{}={},{}", 码, i + 1, 词));
                }
            }
        },

        _ => {
            panic!("请输入正确的模式！")
        }
    }
    fs::write("词库.txt", output.join("\n")).unwrap();
    let _ = std::io::stdin().read_line(&mut mode);
}
