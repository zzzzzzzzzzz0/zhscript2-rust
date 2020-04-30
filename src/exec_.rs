use super::{u_::*, u2_::*, as_ref__};
use std::{io::{self, Write, BufReader, BufRead}, process::{Command, Stdio, ChildStdin, ChildStdout, ChildStderr}};

pub struct Item_ {
	super_:code_::Item1_,
	a_:code_::OL_,
}

impl Item_ {
	pub fn new(kws:&keyword_::List_) -> Self {
		Self {super_:code_::Item1_::new(&kws.exec_), a_:None}
	}

	fn exitcode__(&self, code:Option<i32>, w:&World_, ret:&mut result_::List_) {
		match code {
			Some(i) => {
				ret.add__(i);
				w.dunhao__(ret);
			}
			_ => {
				w.dunhao__(ret);
				ret.add__("Process terminated by signal");
			}
		}
	}
}

#[derive(Default)]
struct Obj_ {
	i_:Option<ChildStdin>,
	o_:Option<ChildStdout>,
	e_:Option<ChildStderr>,
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.kw__()}
	fn add__(&mut self, a:code_::List_) -> Result2_ {
		self.super_.chk_empty__(&a, "缺")?;
		self.a_ = Some(a);
		ok__()
	}
	fn a__(&self) -> code_::ORL_ {t_::some__(&self.a_)}
	fn hello__(&self, gd:code_::Opt_, q:qv_::T_, w:world_::T_, wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_ {
		let w2 = as_ref__!(w);
		let mut ret2 = result_::List_::new();
		t_::o__(&self.a_).hello__(gd, q.clone(), w.clone(), wm, &mut ret2)?;
		
		if ret2.len() >= 1 {
			let end = ret2.len() - 1;
			if ret2.obj_mut__(end, |obj:&mut Obj_| {
				let stdin = obj.i_.as_mut().expect("! stdin");
				let v = ret2.to_vec3__(end);
				for i in &v {
					if let Err(e) = stdin.write_all(i.as_bytes()) {
						eprintln!("{}", e);
					}
				}
			}) {
				return ok__()
			}
		}

		let mut v2 = ret2.to_vec__();
		if v2.is_empty() {return self.super_.err__("空命令")};
		let mut args = str_::split__(&v2.remove(0));
		if args.is_empty() {return self.super_.err__("空命令")};
		let cmd = args.remove(0);
		let mut cmd2 = Command::new(cmd);
		let cmd2 = cmd2.args(args);

		let mut out_src = String::new();
		let mut err_src = String::new();
		//let mut sta_src = String::new();
		let mut mode = "";
		let mut slave_init_src = String::new();
		let mut slave_mode2 = false;
		{
			let cp = clpars_::List_::new2(vec![
				clpars_::Item_::new3("-cd", 1, "工作目录"),
				clpars_::Item_::new3("-env", 2, "环境变量"),
				clpars_::Item_::new3("-出", 1, "重定向之 stdout 用代码"),
				clpars_::Item_::new3("-错", 1, "重定向之 stderr 用代码"),
				clpars_::Item_::new("-堵出"),
				clpars_::Item_::new("-堵错"),
				clpars_::Item_::new("-重定向"),
				clpars_::Item_::new3("-被动者", 1, "后为初始化代码"),
				clpars_::Item_::new2c("-被动者2", 1),
				clpars_::Item_::new("-h|--help"),
				clpars_::Item_::new0(),
			]);
			match cp.for__(&mut v2.into_iter(), |tag, argv, _item, _i3| {
				match tag {
					"-cd" => {cmd2.current_dir(&argv[0]);}
					"-env" => {cmd2.env(&argv[0], &argv[1]);}
					"-出" => out_src.push_str(&argv[0]),
					"-错" => err_src.push_str(&argv[0]),
					"-堵出" => {cmd2.stdout(Stdio::null());}
					"-堵错" => {cmd2.stderr(Stdio::null());}
					"-重定向" => mode = "r",
					"-被动者" | "-被动者2" => {
						slave_init_src = argv[0].to_string();
						mode = "s";
						if tag == "-被动者2" {
							slave_mode2 = true;
						}
					}
					"-h" | "--help" => return 251,
					_ => {
						err_src = [&w2.text__(tag), "无效选项"].concat();
						return 1
					}
				};
				0
			}, |_| 0) {
				0 => {}
				250 => return self.super_.err__("参数不足"),
				_ => return self.super_.err__(&err_src),
			}
		}

		fn f3__(src:&str, args:Option<Vec<String>>, gd:code_::Opt_, q:qv_::T_, w:world_::T_, wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_ {
			let mut q2 = Qv_::new2(Some(q.clone()));
			if let Some(args) = args {
				for s in args {
					q2.args_.add__(
						if    s.ends_with("\r\n") {s[0..s.len() - 2].to_string()}
						else if s.ends_with("\n") {s[0..s.len() - 1].to_string()}
						else {s})
				}
			}
			eval_::hello__(src, gd, qv_::t__(q2), w, wm, ret)
		}

		if mode.is_empty() && (!out_src.is_empty() || !err_src.is_empty()) {
			mode = "r"
		}
		match mode {
			"s" =>
				match cmd2
						.stdin(Stdio::piped())
						.stdout(Stdio::piped())
						.stderr(Stdio::piped())
						.spawn() {
					Ok(mut child) => {
						let stdin = child.stdin.as_mut().expect("! stdin");
						let mut f2__ = |src, args| {
							let mut ret3 = result_::List_::new();
							f3__(src, args, gd, q.clone(), w.clone(), wm, &mut ret3)?;
							let ret2 = ret3.to_vec__();
							for i in ret2 {
								if let Err(e) = stdin.write_all(i.as_bytes()) {
									eprintln!("{}", e);
									break
								}
							}
							ok__()
						};
						f2__(&slave_init_src, None)?;
						if slave_mode2 {
							let mut obj:Obj_ = Default::default();
							obj.i_ = child.stdin;
							obj.o_ = child.stdout;
							obj.e_ = child.stderr;
							ret.add_obj__(Box::new(obj));
						} else {
							let stdout = child.stdout.as_mut().expect("! stdout");
							let stderr = child.stderr.as_mut().expect("! stderr");
							let mut stdout = if out_src.is_empty() {None} else {Some(BufReader::new(stdout))};
							let mut stderr = if err_src.is_empty() {None} else {Some(BufReader::new(stderr))};
							let mut s = String::new();
							loop {
								if let Some(stdout) = &mut stdout {
									match stdout.read_line(&mut s) {
										Ok(siz) => {
											if siz == 0 {break}
											if f2__(&out_src, Some(vec![s.to_string()])).is_err() {}
											s.clear();
										}
										Err(e) => eprintln!("{}", e),
									}
								}
								if let Some(stderr) = &mut stderr {
									match stderr.read_line(&mut s) {
										Ok(siz) => {
											if siz == 0 {break}
											if f2__(&err_src, Some(vec![s.to_string()])).is_err() {}
											s.clear();
										}
										Err(e) => eprintln!("{}", e),
									}
								}
							}
						}
						w2.dunhao__(ret);
					}
					Err(e) => {
						w2.dunhao__(ret);
						ret.add__(e);
					}
				},
			"r" =>
				match cmd2.output() {
					Ok(output) => {
						self.exitcode__(output.status.code(), &w2, ret);
						let mut f__ = |src:&str, v: &[u8]| -> Result2_ {
							if src.is_empty() {
								match String::from_utf8(v.to_vec()) {
									Ok(s) => ret.add__(s),
									Err(e) => eprintln!("{}", e),
								}
							} else if src == "0" {} else {
								let br = BufReader::new(v);
								for line in br.lines() {
									match line {
										Ok(i) => {
											let mut ret3 = result_::List_::new();
											f3__(src, Some(vec![i]), gd, q.clone(), w.clone(), wm, &mut ret3)?;
										}
										Err(e) => eprintln!("{}", e),
									}
								}
							}
							w2.dunhao__(ret);
							ok__()
						};
						if out_src == "0" {
							io::stdout().write_all(&output.stdout).unwrap();
						}
						f__(&out_src, &output.stdout)?;
						if err_src == "0" {
							io::stderr().write_all(&output.stderr).unwrap();
						}
						f__(&err_src, &output.stderr)?;
					}
					Err(e) => {
						w2.dunhao__(ret);
						ret.add__(e);
					}
				},
			_ =>
				match cmd2.status() {
					Ok(st) => {
						self.exitcode__(st.code(), &w2, ret);
					}
					Err(e) => {
						w2.dunhao__(ret);
						ret.add__(e);
					}
				}
		}
		ok__()
	}
}
