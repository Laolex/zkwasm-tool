use clap::Parser;
use std::fs;
use wasmparser::{FuncType, Parser as WasmParser, Payload, ValType};

#[derive(Parser)]
#[command(name = "zkwasm-inspect")]
#[command(about = "🔍 A WASM binary inspector for zkWASM")]
struct Args {
    /// Path to the .wasm file
    path: String,

    /// Show custom sections
    #[arg(short, long)]
    sections: bool,

    /// Enable verbose output for raw section data
    #[arg(short, long)]
    verbose: bool,
}

fn valtype_to_str(val: ValType) -> &'static str {
    match val {
        ValType::I32 => "i32",
        ValType::I64 => "i64",
        ValType::F32 => "f32",
        ValType::F64 => "f64",
        ValType::V128 => "v128",
        ValType::Ref(_) => "ref",
    }
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    if !args.path.ends_with(".wasm") {
        return Err("Input file must have .wasm extension".to_string());
    }
    let wasm_bytes =
        fs::read(&args.path).map_err(|e| format!("Failed to read .wasm file: {}", e))?;
    let parser = WasmParser::new(0);

    println!("🔍 Inspecting WASM file: {}", args.path);

    let mut func_types: Vec<FuncType> = vec![];
    let mut funcs: Vec<u32> = vec![]; // Type indices
    let mut export_names = vec![];
    let mut memory_pages = 0;
    let mut custom_sections = vec![];

    for payload in parser.parse_all(&wasm_bytes) {
        let payload = payload.map_err(|e| format!("Invalid payload: {}", e))?;
        if args.verbose {
            println!("Verbose: Payload = {:?}", payload);
        }
        match payload {
            Payload::TypeSection(reader) => {
                if args.verbose {
                    println!(
                        "Verbose: TypeSection with {} recursive groups",
                        reader.count()
                    );
                }
                for rec_group in reader {
                    let rec_group = rec_group.map_err(|e| format!("Invalid type group: {}", e))?;
                    if args.verbose {
                        println!("Verbose: Recursive group: {:?}", rec_group);
                    }
                    for sub_ty in rec_group.types() {
                        println!("Debug: SubType composite_type: {:?}", sub_ty.composite_type);
                        match &sub_ty.composite_type.inner {
                            wasmparser::CompositeInnerType::Func(func_ty) => {
                                if args.verbose {
                                    println!(
                                        "Verbose: Function type: params={:?}, results={:?}",
                                        func_ty.params(),
                                        func_ty.results()
                                    );
                                }
                                func_types.push(func_ty.clone());
                            }
                            other => {
                                println!("  - Skipping non-function type: {:?}", other);
                            }
                        }
                    }
                }
            }
            Payload::FunctionSection(reader) => {
                if args.verbose {
                    println!("Verbose: FunctionSection with {} functions", reader.count());
                }
                for func in reader {
                    let type_idx = func.map_err(|e| format!("Invalid function type: {}", e))?;
                    if args.verbose {
                        println!("Verbose: Function with type index: {}", type_idx);
                    }
                    funcs.push(type_idx);
                }
            }
            Payload::ExportSection(reader) => {
                if args.verbose {
                    println!("Verbose: ExportSection with {} exports", reader.count());
                }
                for export in reader {
                    let export = export.map_err(|e| format!("Invalid export: {}", e))?;
                    if args.verbose {
                        println!(
                            "Verbose: Export: name={}, kind={:?}, index={}",
                            export.name, export.kind, export.index
                        );
                    }
                    export_names.push(export.name.to_string());
                }
            }
            Payload::MemorySection(reader) => {
                if args.verbose {
                    println!("Verbose: MemorySection with {} memories", reader.count());
                }
                let mut memories = vec![];
                for mem in reader {
                    let mem = mem.map_err(|e| format!("Invalid memory: {}", e))?;
                    if args.verbose {
                        println!("Verbose: Memory: initial={} pages", mem.initial);
                    }
                    memories.push(mem.initial);
                }
                memory_pages = memories.first().copied().unwrap_or(0);
            }
            Payload::ImportSection(reader) => {
                if args.verbose {
                    println!("Verbose: ImportSection with {} imports", reader.count());
                    for import in reader {
                        let import = import.map_err(|e| format!("Invalid import: {}", e))?;
                        println!(
                            "Verbose: Import: module={}, name={}, kind={:?}",
                            import.module, import.name, import.ty
                        );
                    }
                }
            }
            Payload::CustomSection(reader) => {
                if args.verbose {
                    println!(
                        "Verbose: CustomSection: name={}, data_size={}",
                        reader.name(),
                        reader.data().len()
                    );
                }
                custom_sections.push(reader.name().to_string());
            }
            _ => {
                if args.verbose {
                    println!("Verbose: Skipped payload: {:?}", payload);
                }
            }
        }
    }

    println!("\n📦 WASM Summary");
    println!("- Functions: {}", funcs.len());
    for (i, type_idx) in funcs.iter().enumerate() {
        if let Some(f) = func_types.get(*type_idx as usize) {
            let params = f
                .params()
                .iter()
                .map(|t| valtype_to_str(*t))
                .collect::<Vec<_>>()
                .join(", ");
            let results = f
                .results()
                .iter()
                .map(|t| valtype_to_str(*t))
                .collect::<Vec<_>>()
                .join(", ");
            println!("  - func[{i}]: ({params}) -> ({results})");
        } else {
            println!("  - func[{i}]: invalid type index {}", type_idx);
        }
    }
    println!("- Exports: {:?}", export_names);
    println!("- Memory Pages: {}", memory_pages);
    if args.sections {
        println!("- Custom Sections: {:?}", custom_sections);
    }

    for (i, type_idx) in funcs.iter().enumerate() {
        if *type_idx as usize >= func_types.len() {
            return Err(format!(
                "Function {} references invalid type index {}",
                i, type_idx
            ));
        }
        if let Some(f) = func_types.get(*type_idx as usize) {
            let params = f
                .params()
                .iter()
                .map(|t| valtype_to_str(*t))
                .collect::<Vec<_>>()
                .join(", ");
            let results = f
                .results()
                .iter()
                .map(|t| valtype_to_str(*t))
                .collect::<Vec<_>>()
                .join(", ");
            println!("  - func[{i}]: ({params}) -> ({results})");
        }
    }

    Ok(())
}
