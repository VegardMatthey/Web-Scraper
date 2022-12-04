extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let mut file = File::open(r"C:\Users\matth\repo\web\parse\html")?;
    let mut buf_reader = BufReader::new(file);
    let mut text = String::new();
    buf_reader.read_to_string(&mut text)?;

    let week =
        Regex::new(r#"(?m)<h1 class="sage-page-heading__title">[\s]*\d*.-.Uke.(\d*).-.*Report"#)
            .unwrap();
    let day = "torsdag\n";
    let kajabi = "\n";
    let date = Regex::new(r"(?m)Sent.(.*M)").unwrap();
    let purpose = "\n";
    let title =
        Regex::new(r#"(?m)<h1 class="sage-page-heading__title">[\s]*\d*.-.Uke.\d*.-.(.*)Report"#)
            .unwrap();
    let tag = "Alle\n";
    //let delivered = Regex::new(r#"(?m)<a.href="https://help.kajabi.com/hc/en-us/articles/\d*#delivered".target="_blank".rel="noopener".class="[\s]*sage-btn[\s]*sage-btn--subtle[\s]*sage-btn--primary[\s]*sage-btn--icon-right-launch[\s]*">[\s]*<span.class="sage-btn__truncate-text">[\s]*Learn more[\s]*</span>[\s]*</a>[\s]*</div>[\s]*</div>[\s]*</div>[\s]*</div>[\s]*<div.class="sage-card__row[\s]*sage-grid-template-te[\s]*">[\s]*<p.class="t-sage-heading-5">[\s]*(.*)"#).unwrap();
    let opened = Regex::new(r#"(?m)<a href="https://app.kajabi.com/admin/email_broadcasts/\d*/report\?by_event_type=opened">Opened</a>[\S\s]*Opened[\S\s]*With a recent update to mail privacy, some email clients, like[\s]*Apple, no longer report opens accurately. Focusing on clicks is a better[\s]*strategy.[\s]*</div>[\s]*<div class="sage-popover__actions">[\s]*<a href="https://help.kajabi.com/hc/en-us/articles/4407133043099" target="_blank" rel="noopener" class="[\s]*sage-btn[\s]*sage-btn--subtle[\s]*sage-btn--primary[\s]*sage-btn--icon-right-launch[\s]*">[\s]*<span class="sage-btn__truncate-text">[\s]*Learn more[\s]*</span>[\s]*</a>[\s]*</div>[\s]*</div>[\s]*</div>[\s]*</div>[\s]*<div class="sage-card__row[\s]*sage-grid-template-te[\s]*">[\s]*<p class="t-sage-heading-5">[\s]*(.*%)[\s]*<span class="t-sage-body-xsmall t-sage--color-charcoal-100">[\s]*(.* total)"#).unwrap();
    let clicked = Regex::new(r#"(?m)<a.href="https://help.kajabi.com/hc/en-us/articles/\d*#clicked".target="_blank".rel="noopener".class="[\s]*sage-btn[\s]*sage-btn--subtle[\s]*sage-btn--primary[\s]*sage-btn--icon-right-launch[\s]*">[\s]*<span.class="sage-btn__truncate-text">[\s]*Learn more[\s]*</span>[\s]*</a>[\s]*</div>[\s]*</div>[\s]*</div>[\s]*</div>[\s]*<div.class="sage-card__row[\s]*sage-grid-template-te[\s]*">[\s]*<p.class="t-sage-heading-5">[\s]*(.*)"#).unwrap();
    let hard_bounced = Regex::new(r#"(?m)<a.href="https://help.kajabi.com/hc/en-us/articles/\d*#bounced".target="_blank".rel="noopener".class="[\s]*sage-btn[\s]*sage-btn--subtle[\s]*sage-btn--primary[\s]*sage-btn--icon-right-launch[\s]*">[\s]*<span.class="sage-btn__truncate-text">[\s]*Learn more[\s]*</span>[\s]*</a>[\s]*</div>[\s]*</div>[\s]*</div>[\s]*</div>[\s]*<div.class="sage-card__row[\s]*sage-grid-template-te[\s]*">[\s]*<p.class="t-sage-heading-5">[\s]*(.*)"#).unwrap();
    let unsubscribed = Regex::new(r#"(?m)<a href="https://help.kajabi.com/hc/en-us/articles/\d*#unsubscribed" target="_blank" rel="noopener" class="[\s]*sage-btn[\s]*sage-btn--subtle[\s]*sage-btn--primary[\s]*sage-btn--icon-right-launch[\s]*">[\s]*<span class="sage-btn__truncate-text">[\s]*Learn more[\s]*</span>[\s]*</a>[\s]*</div>[\s]*</div>[\s]*</div>[\s]*</div>[\s]*<div class="sage-card__row[\s]*sage-grid-template-te[\s]*">[\s]*<p class="t-sage-heading-5">[\s]*(.*%)"#).unwrap();
    let sent = Regex::new(r#"(?m)<a.href="https://help.kajabi.com/hc/en-us/articles/\d*#sent".target="_blank".rel="noopener".class="[\s]*sage-btn[\s]*sage-btn--subtle[\s]*sage-btn--primary[\s]*sage-btn--icon-right-launch[\s]*">[\s]*<span.class="sage-btn__truncate-text">[\s]*Learn more[\s]*</span>[\s]*</a>[\s]*</div>[\s]*</div>[\s]*</div>[\s]*</div>[\s]*<div.class="sage-card__row[\s]*sage-grid-template-te[\s]*">[\s]*<p.class="t-sage-heading-5">[\s]*(.*)"#).unwrap();

    let list: Vec<Regex> = vec![
        week,
        title,
        //delivered,
        opened,
        clicked,
        hard_bounced,
        unsubscribed,
        sent,
    ];

    let mut result_file = File::create("result.txt")?;

    let mut current: String;

    let mut data: Vec<String> = Vec::new();

    let mut i = 0;
    for item in list {
        let caps = item.captures_iter(&text);
        if i == 1 {
            data.push(day.to_string());
            data.push(kajabi.to_string());
            data.push("date".to_string());
            data.push(purpose.to_string());
        }
        if i == 2 {
            data.push(tag.to_string());
        }
        for mat in caps {
            current = String::from(mat.get(1).map_or("", |m| m.as_str())) + "\n";
            //print!("{}", current);
            data.push(current);
            //result_file
            //    .write_all(current.as_bytes())
            //    .expect("Unable to write data");
        }
        i += 1;
    }

    file = File::open(r"C:\Users\matth\repo\web\parse\overview")?;
    buf_reader = BufReader::new(file);
    buf_reader.read_to_string(&mut text)?;

    let caps = date.captures(&text);
    for mat in caps {
        current = String::from(mat.get(1).map_or("", |m| m.as_str()));
        data[3] = current.to_string() + "\n";
    }

    for datapoint in data {
        print!("{}", datapoint);
        result_file
            .write_all(datapoint.as_bytes())
            .expect("Unable to write data");
    }

    Ok(())
}
