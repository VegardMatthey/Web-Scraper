extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let emails: u16 = 25;
    let data_path = r"C:\Users\matth\repo\web\parse\data\";

    for x in 0..emails {
        let file = File::open(data_path.to_owned() + "html" + &(x.to_string())[..] + ".txt")?;
        let result_file =
            File::create(data_path.to_owned() + "result" + &(x.to_string())[..] + ".txt")?;
        let overview = File::open(data_path.to_owned() + "overview.txt")?;
        parse(x, emails, file, result_file, overview).expect("failed");
    }
    Ok(())
}

fn parse(
    index: u16,
    emails: u16,
    file: File,
    mut result_file: File,
    overview: File,
) -> std::io::Result<()> {
    let mut buf_reader = BufReader::new(file);
    let mut text = String::new();
    buf_reader.read_to_string(&mut text)?;

    //let week =
    //    Regex::new(r#"(?m)<h1 class="sage-page-heading__title">[\s]*\d*.-.Uke.(\d*).-.*Report"#)
    //        .unwrap();
    let week = "\n";
    let day = "torsdag\n";
    let kajabi = "\n";
    let date = Regex::new(r"(?m)Sent.(.*M)").unwrap();
    let purpose = "\n";
    //let title = Regex::new(r#"(?m)Email Campaigns[\s]*</a>[\s]*</li>[\s]*<li class="sage-breadcrumbs__item">[\s]*<a href="" class="[\s]*sage-breadcrumbs__link[\s]*sage-breadcrumbs__link--current[\s]*" aria-disabled="true">[\s]*(.*)[\s]*</a>"#).unwrap();
    let title = Regex::new(r"(?m)Close[\s]*(.*)Report[\s]*Preview.<https://app.kajabi.com/admin/email_broadcasts/\d*/preview>").unwrap();

    let tag = "Alle\n";
    //let opened = Regex::new(r#"(?m)<a href="https://app.kajabi.com/admin/email_broadcasts/\d*/report\?by_event_type=opened">Opened</a>[\S\s]*Opened[\S\s]*With a recent update to mail privacy, some email clients, like[\s]*Apple, no longer report opens accurately. Focusing on clicks is a better[\s]*strategy.[\s]*</div>[\s]*<div class="sage-popover__actions">[\s]*<a href="https://help.kajabi.com/hc/en-us/articles/4407133043099" target="_blank" rel="noopener" class="[\s]*sage-btn[\s]*sage-btn--subtle[\s]*sage-btn--primary[\s]*sage-btn--icon-right-launch[\s]*">[\s]*<span class="sage-btn__truncate-text">[\s]*Learn more[\s]*</span>[\s]*</a>[\s]*</div>[\s]*</div>[\s]*</div>[\s]*</div>[\s]*<div class="sage-card__row[\s]*sage-grid-template-te[\s]*">[\s]*<p class="t-sage-heading-5">[\s]*(.*%)[\s]*<span class="t-sage-body-xsmall t-sage--color-charcoal-100">[\s]*(.* total)"#).unwrap();
    let opened = Regex::new(r"(?m)Opened[\s]*With a recent update to mail privacy, some email clients, like Apple, no[\s]*longer report opens accurately. Focusing on clicks is a better strategy.[\s]*Learn more <https://help.kajabi.com/hc/en-us/articles/\d*>[\s]*(.*%)").unwrap();
    //let clicked = Regex::new(r#"(?m)<a.href="https://help.kajabi.com/hc/en-us/articles/\d*#clicked".target="_blank".rel="noopener".class="[\s]*sage-btn[\s]*sage-btn--subtle[\s]*sage-btn--primary[\s]*sage-btn--icon-right-launch[\s]*">[\s]*<span.class="sage-btn__truncate-text">[\s]*Learn more[\s]*</span>[\s]*</a>[\s]*</div>[\s]*</div>[\s]*</div>[\s]*</div>[\s]*<div.class="sage-card__row[\s]*sage-grid-template-te[\s]*">[\s]*<p.class="t-sage-heading-5">[\s]*(.*)"#).unwrap();
    let clicked = Regex::new(r"(?m)Clicked[\s]*An email is considered clicked when someone clicks any link inside of[\s]*the email.[\s]*Learn more <https://help.kajabi.com/hc/en-us/articles/\d*#clicked>[\s]*(.*%)").unwrap();
    //let hard_bounced = Regex::new(r#"(?m)<a.href="https://help.kajabi.com/hc/en-us/articles/\d*#bounced".target="_blank".rel="noopener".class="[\s]*sage-btn[\s]*sage-btn--subtle[\s]*sage-btn--primary[\s]*sage-btn--icon-right-launch[\s]*">[\s]*<span.class="sage-btn__truncate-text">[\s]*Learn more[\s]*</span>[\s]*</a>[\s]*</div>[\s]*</div>[\s]*</div>[\s]*</div>[\s]*<div.class="sage-card__row[\s]*sage-grid-template-te[\s]*">[\s]*<p.class="t-sage-heading-5">[\s]*(.*)"#).unwrap();
    let hard_bounced = Regex::new(r"(?m)Bounced[\s]*An email is considered bounced when it is rejected by an email server,[\s]*but not all bounces are the same.[\s]*Learn more <https://help.kajabi.com/hc/en-us/articles/\d*#bounced>[\s]*(.*%)").unwrap();
    //let unsubscribed = Regex::new(r#"(?m)<a href="https://help.kajabi.com/hc/en-us/articles/\d*#unsubscribed" target="_blank" rel="noopener" class="[\s]*sage-btn[\s]*sage-btn--subtle[\s]*sage-btn--primary[\s]*sage-btn--icon-right-launch[\s]*">[\s]*<span class="sage-btn__truncate-text">[\s]*Learn more[\s]*</span>[\s]*</a>[\s]*</div>[\s]*</div>[\s]*</div>[\s]*</div>[\s]*<div class="sage-card__row[\s]*sage-grid-template-te[\s]*">[\s]*<p class="t-sage-heading-5">[\s]*(.*%)"#).unwrap();
    let unsubscribed = Regex::new(r"(?m)Unsubscribed[\s]*An email address is unsubscribed when the recipient indicates they no[\s]*longer wish to receive emails from a sender.[\s]*Learn more[\s]*<https://help.kajabi.com/hc/en-us/articles/\d*#unsubscribed>[\s]*(.*%)").unwrap();
    //let sent = Regex::new(r#"(?m)<a.href="https://help.kajabi.com/hc/en-us/articles/\d*#sent".target="_blank".rel="noopener".class="[\s]*sage-btn[\s]*sage-btn--subtle[\s]*sage-btn--primary[\s]*sage-btn--icon-right-launch[\s]*">[\s]*<span.class="sage-btn__truncate-text">[\s]*Learn more[\s]*</span>[\s]*</a>[\s]*</div>[\s]*</div>[\s]*</div>[\s]*</div>[\s]*<div.class="sage-card__row[\s]*sage-grid-template-te[\s]*">[\s]*<p.class="t-sage-heading-5">[\s]*(.*)"#).unwrap();
    let sent = Regex::new(r"(?m)Sent[\s]*This number will always be the number of contacts that you originally[\s]*intended to send an email to.[\s]*Learn more[\s]*<https://help.kajabi.com/hc/en-us/articles/\d*#sent>[\s]*(.*)").unwrap();

    let list: Vec<Regex> = vec![
        title,
        //delivered,
        opened,
        clicked,
        hard_bounced,
        unsubscribed,
        sent,
    ];

    let mut current: String;

    let mut data: Vec<String> = Vec::new();

    let mut i = 1;
    data.push(week.to_string());
    for item in list {
        let caps = item.captures_iter(&text);
        if i == 1 {
            data.push(day.to_string());
            data.push(kajabi.to_string());
            data.push("\n".to_string());
            data.push(purpose.to_string());
        }
        if i == 2 {
            data.push(tag.to_string());
        }
        for mat in caps {
            current = String::from(mat.get(1).map_or("", |m| m.as_str())) + "\n";
            data.push(current);
        }
        i += 1;
    }

    buf_reader = BufReader::new(overview);
    buf_reader.read_to_string(&mut text)?;

    let caps = date.captures_iter(&text);
    i = 0;
    for mat in caps {
        if i == emails - 1 - index {
            current = String::from(mat.get(1).map_or("", |m| m.as_str()));
            data[3] = current.to_string() + "\n";
        }
        i += 1;
    }

    for datapoint in data {
        //print!("{}", datapoint);
        result_file
            .write_all(datapoint.as_bytes())
            .expect("Unable to write data");
    }

    Ok(())
}
