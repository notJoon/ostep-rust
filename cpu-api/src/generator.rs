use std::fs::File;
use std::io::Write;

use clap::Parser;

use crate::seed;

// Boilerplate code for `.c` files used by both readable/runnable code versions
struct Boilerplate {
    fd: File,
}

impl Boilerplate {
    fn new(fd: File) -> Self {
        Self { fd }
    }

    fn init(&mut self) {
        self.fd.write(b"
#include <assert.h>\n
#include <stdio.h>\n
#include <stdlib.h>\n
#include <string.h>\n
#include <sys/time.h>\n
#include <sys/wait.h>\n
#include <unistd.h>\n\n

void wait_or_die() {\n
    int rc = wait(NULL);\n
    assert(rc > 0);\n
}\n\n

int for_or_die() {\n
    int rc = fork();\n
    assert(rc >= 0);\n
    return rc;\n
}\n\n
").expect("Failed to write to file");
    }

    fn init_runnable(&mut self) {
        self.init();
        self.fd.write(b"
#define TimeGetSeconds() ({ struct timeval t; int rc = gettimeofday(&t, NULL); assert(rc == 0); (double) t.tv_sec + (double) t.tv_usec/1e6; }) \n\n

double t_start;\n\n

struct pid_map {\n
    int pid;\n
    char name[10];\n
    struct pid_map *next; \n
};\n\n

struct pid_map *head = NULL;\n\n

void Space(char c) {\n
    int i;\n
    for (i = 0; i < 5 * (c - \'a\'); i++) {\n
        printf(\" \");\n
    }\n
}\n\n

char *Lookup(int pid) {\n
    struct pid_map *curr = head;\n
    while (curr) {\n
        if (curr->pid == pid) {\n
            return curr->name;\n
        }\n
        curr = curr->next;\n
    }\n
    return NULL;\n
}\n\n

void Record(int pid, char *m) {\n
    struct pid_map *n = malloc(sizeof(struct pid_map));\n
    assert(n);\n
    n->pid = pid;\n
    strcpy(n->name, m);\n
    n->next = head;\n
    head = n;\n
}\n\n

void Wait(char *m) {\n
    int rc = wait(NULL);\n
    assert(rc > 0);\n
    double t = TimeGetSeconds() - t_start;\n
    printf(\"&3d\", (int)t);\n
    Space(m[0]);\n
    char *n = Lookup(rc);\n
    assert(n != NULL);\n
    printf(\"%s<-%s\\n\", m, n);\n
}\n\n

void Sleep(int s, char *m) {\n
    sleep(s);\n
    double t = TimeGetSeconds() - t_start;\n
    printf(\"%3d %s\", (int)t, m);\n
}\n\n

void Fork(char *p, char *c) {\n
    double t = TimeGetSeconds() - t_start;\n
    printf(\"%3d\", (int)t);\n
    Space(p[0]);\n
    printf(\"%s->%s\\n\", p, c);\n
}\n\n

void __Begin(char *m) {\n
    double t = TimeGetSeconds() - t_start;\n
    printf(\"%3d\", (int)t);\n
    Space(m[0]);\n
    printf(\"%s+\\n\", m);\n
}\n\n

void __End(char *m) {\n
    double t = TimeGetSeconds() - t_start;\n
    printf(\"%3d\", (int)t);\n
    Space(m[0]);\n
    printf(\"%s-\\n\", m);\n
}\n\n

#define Begin(m) { __Begin(m); }\n
#define End(m) { __End(m); exit(0); }\n
        ").expect("Fail to write to file");
    }

    fn finish(&mut self) {
        self.fd.write(
b"    return 0;\n
}\n
").expect("Failed to write to file");
    }

    fn main(&mut self) {
        self.main();
        self.fd.write(
b"      int rc;\n
        t_start = TimeGetSeconds();\n
"
        ).expect("Failed to write to file");
    }
}

struct CodeGeneratorReadable {
    fd: File,
    boiler: Boilerplate,
    tab_level: u32,
    out_file: String,
    actions: Vec<String>,
}

impl CodeGeneratorReadable {
    fn new(self, fd: File, out_file: String, actions: Vec<String>) -> Self {
        let out_file = out_file + ".c";
        let actions = actions;
        let tab_level = 1;
        let boiler = Boilerplate::new(fd);

        Self {
            fd,
            boiler,
            tab_level,
            out_file,
            actions,
        }
    }
}