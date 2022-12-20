use anyhow::Result;
use bwterraingen::{Rules, Wave2};
use std::env;
use tracing::info;
use tracing_log::LogTracer;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::EnvFilter;

fn main() -> Result<()> {
    setup_logging()?;

    let args: Vec<String> = env::args().collect();
    anyhow::ensure!(args.len() >= 1);

    let width = (&args[1]).parse::<usize>()?;
    let height = (&args[2]).parse::<usize>()?;
    let dest = args[3].clone();

    anyhow::ensure!(args.len() > 4);

    // let rules = args[4..]
    //     .into_iter()
    //     .filter_map(|arg| {
    //         info!("filename: {arg}");
    //         //let chk = get_chk_from_mpq_filename(arg.clone()).unwrap();
    //         let chk = std::fs::read(arg).unwrap();
    //         Some(get_rules_from_chk(&chk).unwrap())
    //     })
    //     .reduce(|x, y| x.combine(&y).unwrap())
    //     .unwrap();

    let rules = args[4..]
        .into_iter()
        .map(|arg| {
            info!("filename: {arg}");
            //let chk = get_chk_from_mpq_filename(arg.clone()).unwrap();
            let rules: Rules =
                serde_json::from_str(std::fs::read_to_string(arg).unwrap().as_str()).unwrap();
            rules
        })
        .reduce(|x, y| x.combine(&y).unwrap())
        .unwrap();

    let mut wave = Wave2::new(width as isize, height as isize, &rules, None);

    wave.logical_conclusion(
        &|_x| {
            // x.print_wave();
            // info!(
            //     "non-null tiles: {:6} / {:6}",
            //     x.render().into_iter().filter(|x| *x != 15).count(),
            //     width * height
            // );
        },
        500,
        4,
    )?;

    // info!("final wave:");
    // wave.print_wave();

    let render = wave.render();

    for y in 0..height {
        let mut output = String::new();
        for x in 0..width {
            output = format!("{}{:6}", output, render[(x + y * width) as usize]);
        }
        info!("{}", output);
    }

    let output_chk = bwterraingen::create_chk_from_wave(&wave.render(), rules.era, width, height);

    std::fs::write(dest, output_chk)?;

    anyhow::Ok(())
}

fn setup_logging() -> Result<()> {
    // enable console_subcriber only in debug build because it consumes so much memory it breaks the server
    if cfg!(debug_assertions) {
        //console_subscriber::init();
    }

    LogTracer::init().expect("Failed to set logger");

    let filter = EnvFilter::from_default_env();
    let subscriber = tracing_subscriber::fmt()
        // filter spans/events with level TRACE or higher.
        .with_env_filter(filter)
        .with_span_events(FmtSpan::CLOSE)
        .with_file(true)
        .with_target(false)
        .with_line_number(true)
        .with_level(false)
        // build but do not install the subscriber.
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    anyhow::Ok(())
}
