# 实验报告

## 功能实现简述


## 简答作业
1. 正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。 请同学们可以自行测试这些内容（运行 三个 bad 测例 (ch2b_bad_*.rs) ）， 描述程序出错行为，同时注意注明你使用的 sbi 及其版本。

   - 使用 `[rustsbi] RustSBI version 0.2.2, adapting to RISC-V SBI v1.0.0`，QEMU 版本是 9.2.4，系统环境为 WSL 2 下的Ubuntu-24.04。
   - ch2b_bad_address.rs：尝试在地址 0x0 写入数据，而这通常是被保护的地址范围，类似 Windows 给程序留下的“防撞墙”一样，当尝试访问一定范围的地址空间就视为空指针等行为发生。
   - ch2b_bad_instructions.rs：内联汇编指令 `sret`，这明显不行。根据 ch2 文档，在 user app 运行时，`status` 的 `SPP` 字段会被修改为 U，而 `sret` 只能在内核特权级 S 时由内核执行。
   - ch2b_bad_register.rs：内敛汇编指令，尝试获取寄存器 sstatus 的值， 但同样，user app 运行时，不可能操作需要特权级 S 才能操作的寄存器 sstatus。

   


2. 深入理解 [trap.S](https://github.com/LearningOS/rCore-Camp-Code-2025S/blob/ch3/os/src/trap/trap.S) 中两个函数 `__alltraps` 和 `__restore` 的作用，并回答如下问题:

   1. L40：刚进入 `__restore` 时，`sp` 代表了什么值。请指出 `__restore` 的两种使用情景。

      - sp 代表 kernel stack 的栈指针。
      - 情景一：通过 `__restore` 切入 app 并开始执行
      - 情景二：处理完 trap 返回特权级 U 时恢复寄存器 
   
   2. L43-L48：这几行汇编代码特殊处理了哪些寄存器？这些寄存器的的值对于进入用户态有何意义？请分别解释。
   
      ```
      ld t0, 32*8(sp)
      ld t1, 33*8(sp)
      ld t2, 2*8(sp)
      csrw sstatus, t0
      csrw sepc, t1
      csrw sscratch, t2
      ```
   
      - 处理了 `sstatus`、`sepc` 和 `sscratch` 寄存器。
   
      - `sstatus` 寄存器各字段对于用户态的意义：
   
        SPP: 指示当前模式
   
        SIE: 用户态时可以忽略

        SPIE: 中断可用状态

        UIE: U 模式中断使能，当且仅当 UIE 置位且 hart 处于 U 模式
   
        UPIE: 表示处理 U 模式 trap 前，U 模式中断是否可用。可忽略，忽略时 UIE、UPIE 固定为零

        PUM: 修改 S 模式加载、存储和取指令访问虚拟内存权限。

      - `sepc` ：

        sepc[0] 始终为0。S 态程序异常计数器，指示下一条命令。
   
      - `sscratch` :

        sscratch用于保存指向hart-local supervisor上下文的指针。在trap处理程序的开头，sscratch与用户寄存器交换，以提供初始工作寄存器。
   
   3. L50-L56：为何跳过了 `x2` 和 `x4`？
   
      ```
      ld x1, 1*8(sp)
      ld x3, 3*8(sp)
      .set n, 5
      .rept 27
         LOAD_GP %n
         .set n, n+1
      .endr
      ```
   
      - **x2 (sp)**：栈指针寄存器需要特殊处理。在恢复其他寄存器期间，sp 必须保持指向当前的内核栈，只有在所有寄存器恢复完成后才能恢复 sp
      - **x4 (tp)**：线程指针寄存器通常在整个陷阱处理过程中保持不变，或者由操作系统单独管理，不需要在此处恢复
   
   4. L60：该指令之后，`sp` 和 `sscratch` 中的值分别有什么意义？
   
      ```
      csrrw sp, sscratch, sp
      ```
   
      - **sp**：现在指向**用户栈**（原来 sscratch 中保存的值）
      - **sscratch**：现在保存**内核栈指针**（原来 sp 的值）
   
   5. `__restore`：中发生状态切换在哪一条指令？为何该指令执行之后会进入用户态？
   
      - `sret` 后状态改变
        1. **特权级别切换**：根据 sstatus.SPP 位的值决定目标特权级别（0=用户态，1=supervisor态）
        2. **程序计数器更新**：PC 被设置为 sepc 中保存的地址
        3. **中断使能恢复**：sstatus.SIE 被设置为 sstatus.SPIE 的值，然后 SPIE 置为1
        4. **SPP 位清零**：sstatus.SPP 被设置为0（用户态）
   
   6. L13：该指令之后，`sp` 和 `sscratch` 中的值分别有什么意义？
   
      ```
      csrrw sp, sscratch, sp
      ```
   
      - **sp**：指向用户栈，供用户态程序使用
      - **sscratch**：指向内核栈，为下次陷阱处理保存内核栈地址
   
   7. 从 U 态进入 S 态是哪一条指令发生的？
   
      触发 trap，在 trap_handler 中完成。


## 荣誉准测

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

暂无交流对象

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

参考了 rCore 文档：
- https://learningos.cn/rCore-Camp-Guide-2025S
- https://rcore-os.cn/rCore-Tutorial-Book-v3

以及在线 RISC-V 指令集检索网站和关于 RISC-V 特权级寄存器的博客：
- https://ai-embedded.com/risc-v/riscv-isa-manual/
- https://blog.csdn.net/Pandacooker/article/details/116423306

另外还有关于 Rust unsafe 使用方式的文档：

- https://course.rs/
- https://nomicon.purewhite.io/


3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。