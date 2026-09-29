# Excerpts from the frozen source compiled with Emscripten 6.0.6 -O2.

checkstring3:                           # @checkstring3
	.functype	checkstring3 (i32) -> (i32)
	.local  	i32, i32, i32, i32, i32, i32, i32, i32, i32
# %bb.0:
	global.get	__stack_pointer
	i32.const	128
	i32.sub 
	local.tee	1
	global.set	__stack_pointer
	call	cur_lang
	local.set	2
	local.get	1
	i32.const	64
	i32.add 
	local.get	0
	i32.const	824
	i32.add 
	local.tee	3
	i32.const	60
	call	Xstrncpy
	drop
	local.get	0
	call	checkstring4
	local.set	4
	block   	
	block   	
	block   	
	local.get	0
	i32.load8_s	824
	local.tee	5
	i32.const	42
	i32.eq  

checkstring4:                           # @checkstring4
	.functype	checkstring4 (i32) -> (i32)
	.local  	i32, i32, i32, i32, i32
# %bb.0:
	global.get	__stack_pointer
	i32.const	192
	i32.sub 
	local.tee	1
	global.set	__stack_pointer
	block   	
	local.get	0
	call	checkword
	local.tee	2
	i32.const	0
	i32.gt_s
	br_if   	0                               # 0: down to label71
# %bb.1:
	local.get	1
	local.get	0
	i32.const	824
	i32.add 
	local.tee	3
	i32.const	60
	call	Xstrncpy
	drop
	local.get	1
	i32.const	128
	i32.add 
	local.get	1
	i32.const	60
	call	Xstrncpy
	drop
	local.get	1
	i32.const	64
	i32.add 
	local.get	1
	i32.const	60
	call	Xstrncpy
	drop
	local.get	1
	i32.const	64
	i32.add 
	call	stripacc
	drop
