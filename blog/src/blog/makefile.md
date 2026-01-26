<!--
date: "2022-12-01"
tags: [programming]
title: "Makefile for Small Stuff"
summary: "A step-by-step guide to writing Makefiles for small C/C++ projects."
-->

# Makefile for Small Stuff

When you write code in C or C++, you will need to compile and link your stuff. For a few files this could be done by hand; however, for slightly-larger small projects it is better to use a **Make**.

> Make is a build automation tool that automatically builds executable programs and libraries from source code by reading files called Makefiles which specify how to derive the target program.

The quote above is from [Wikipedia](<https://en.wikipedia.org/wiki/Make_(software)>), and is pretty explanatory. To build a program, you just have a Makefile that specifies which program should include what, use which compiler with which flags and such (i.e. a recipe) and then you just type `make`; et viola, your executable binary is created.

The absolute beauty of Make is that it only compiles what it deems necessary; for example if you already compiled all your files but later changed just one file, it will only compile that one and link everything with the new object files. This greatly speeds things up.

_NOTE:_ For bigger projects with many folder and subfolders it may be better to use tools like [CMake](https://cmake.org/), which is a tool that creates Makefile's for you!

## A Generic Makefile

Let us build a Makefile, where I will explain everything step by step. The file will be quite verbose, but you can hardcode things as you see fit. The example will be for a small C++ project.

(1) We will first specify the binary targets, directories and some extensions. C++ files have `.cpp` extension, and I like to separate template definitions from the header files so I have `.tpp` files for that.

```makefile
# Binary target
TGTDIR  := bin
TARGET  ?= myapp
TESTTGT  = test

# Code extensions
SRCEXT	:= .cpp
INCEXT	:= .hpp
TPLEXT	:= .tpp
OBJEXT	:= .o

# Code directories
SRCDIR	:= src
INCDIR	:= include
TPLDIR  := templates
OBJDIR	:= build
```

(2) We will now specify the compiler & linker options, and prepare our source codes to be used in the recipe.

```makefile
# Compiler and linker
CC      := g++
LD      := g++
CCMACRO ?=
LDFLAGS	:= -Llib -fopenmp
CCFLAGS	:= -O3 -Wall -Wextra -pedantic -Wno-sign-compare $(CCMACRO)

# Code files
INCS     = $(shell find $(INCDIR) -type f -name '*$(INCEXT)')
INCS    += $(shell find $(TPLDIR) -type f -name '*$(TPLEXT)')
SRCS     = $(shell find $(SRCDIR) -type f -name '*$(SRCEXT)')
OBJS     = $(patsubst $(SRCDIR)/%,$(OBJDIR)/%,$(SRCS:.cpp=.o))
```

(3) Now we will specify our linking and compiling rules.

```makefile
# First rule
all: $(TARGET) | $(TGTDIR)
    @mv $(TARGET) $(TGTDIR)/

# Linking
$(TARGET): $(OBJS)
    $(LD) $(LDFLAGS) -L$(OBJDIR) -o $@ $^

# Compiling
$(OBJS): $(OBJDIR)/%$(OBJEXT) : $(SRCDIR)/%$(SRCEXT) | $(OBJDIR)
    $(CC) $(CCFLAGS) -I$(INCDIR) -I$(TPLDIR) -c -o $@ $?

# Objects directory
$(OBJDIR):
    mkdir -p $(OBJDIR)

# Target directory
$(TGTDIR):
    mkdir -p $(TGTDIR)
```

Makefile calls the first rule (the one at the top). In this case, that is `all` rule and it depends on the target. The `linking` rule simply links the object files, which are coming from the `compiling` rule. The `OBJDIR` and `TGTDIR` rules are there to ensure that the object and target directories exist.

The Makefile so far actually builds a binary, but you can have more rules for utility! I like to have a few such as:

- `run` to run the program. I usually have `$(ARGUMENTS)` in this rule, and you can set this from the command line as you call this rule.
- `validate` to automatically run tests.
- `clean` to clean the objects.
- `tests` rule to create a test binary, with special arguments.
- `again` is simply `make clean && make`; there are times where I make changes often and some of them require recompiling the entire thing.
- `show` is just for me to see which files are being used in Makefile. This is mostly diagnostic.

```makefile
# Run with arguments
run:
    @./$(TGTDIR)/$(TARGET) $(ARGUMENTS)

# Run tests
validate:
    @./$(TGTDIR)/$(TESTTGT)

# Clean objects and binaries
clean:
    rm -f $(OBJS)

# Create a test binary
tests:
    @make clean
    # defines IS_TESTING macro in the code,
    # to change which codes get compiled
    @make CCMACRO="-DIS_TESTING=1" TARGET=$(TESTTGT)

# Clean and make again
again:
    @make clean
    @make

# Diagnostic to show files
show:
    @echo "Sources: $(SRCS)"
    @echo "Includes: $(INCS)"
    @echo "Objects: $(OBJS)"
```

Notice that none of these rules depend on anything. In such cases, we have to specify these as _phony_ rules:

```makefile
.PHONY: run validate clean again test show
```

That is it! The entire Makefile is just all of these code snippets added on top of eachother:

```makefile
# Binary target
TGTDIR  := bin
TARGET  ?= myapp
TESTTGT  = test

# Code extensions
SRCEXT	:= .cpp
INCEXT	:= .hpp
TPLEXT	:= .tpp
OBJEXT	:= .o

# Code directories
SRCDIR	:= src
INCDIR	:= include
TPLDIR  := templates
OBJDIR	:= build

# Compiler and linker
CC      := g++
LD      := g++
CCMACRO ?=
LDFLAGS	:= -Llib -fopenmp
CCFLAGS	:= -O3 -Wall -Wextra -pedantic -Wno-sign-compare $(CCMACRO)

# Code files
INCS  = $(shell find $(INCDIR) -type f -name '*$(INCEXT)')
INCS += $(shell find $(TPLDIR) -type f -name '*$(TPLEXT)')
SRCS  = $(shell find $(SRCDIR) -type f -name '*$(SRCEXT)')
OBJS  = $(patsubst $(SRCDIR)/%,$(OBJDIR)/%,$(SRCS:.cpp=.o))

# First rule
all: $(TARGET) | $(TGTDIR)
    @mv $(TARGET) $(TGTDIR)/

# Linking
$(TARGET): $(OBJS)
    $(LD) $(LDFLAGS) -L$(OBJDIR) -o $@ $^

# Compiling
$(OBJS): $(OBJDIR)/%$(OBJEXT) : $(SRCDIR)/%$(SRCEXT) | $(OBJDIR)
    $(CC) $(CCFLAGS) -I$(INCDIR) -I$(TPLDIR) -c -o $@ $?

# Objects directory
$(OBJDIR):
    mkdir -p $(OBJDIR)

# Target directory
$(TGTDIR):
    mkdir -p $(TGTDIR)

# Run with arguments
run:
    @./$(TGTDIR)/$(TARGET) $(ARGUMENTS)

# Run tests
validate:
    @./$(TGTDIR)/$(TESTTGT)

# Clean objects and binaries
clean:
    rm -f $(OBJS)

# Create a test binary
tests:
    @make clean
    @make CCMACRO="-DIS_TESTING=1" TARGET=$(TESTTGT)

# Clean and make again
again:
    @make clean
    @make

# Diagnostic to show files
show:
    @echo "Sources: $(SRCS)"
    @echo "Includes: $(INCS)"
    @echo "Objects: $(OBJS)"

.PHONY: all run validate clean again test show
```
