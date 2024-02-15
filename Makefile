CARGO_FLAGS ?= --release

MACHINE := $(shell getconf LONG_BIT)
ifeq ($(MACHINE), 64)
LIB ?= lib64
else
LIB ?= lib
endif

ifdef MINGW_PREFIX
export MSYS2_ARG_CONV_EXCL=*
export PREFIX=$(MINGW_PREFIX)
else
export PREFIX=/usr
endif
export PKG_CONFIG_PATH+=:$(PWD)/inst/$(PREFIX)/$(LIB)/pkgconfig
export GI_TYPELIB_PATH+=:$(PWD)/inst/$(PREFIX)/$(LIB)/girepository-1.0
export LD_LIBRARY_PATH=$(PWD)/inst/$(PREFIX)/$(LIB)

HEADER = inst/$(PREFIX)/include/gtk4-rlottie/gtk4-rlottie.h
GIR = inst/$(PREFIX)/share/gir-1.0/Lottie-0.1.gir
TYPELIB = inst/$(PREFIX)/$(LIB)/girepository-1.0/Lottie-0.1.typelib
VAPI = inst/$(PREFIX)/share/vala/vapi/gtk4-rlottie.vapi

RUST_SOURCES = $(shell find src)

all: $(GIR) $(TYPELIB) $(VAPI)

$(HEADER): $(RUST_SOURCES)
	cargo cinstall $(CARGO_FLAGS) --destdir=inst --prefix=$(PREFIX) --libdir=$(PREFIX)/$(LIB)

$(GIR): $(HEADER)
	mkdir -p $(@D)
	g-ir-scanner -v --warn-all \
		--namespace Lottie --nsversion=0.1 \
		--identifier-prefix Lottie \
		--c-include "gtk4-rlottie.h" \
		-Iinst/$(PREFIX)/include \
		-Linst/$(PREFIX)/$(LIB) \
		--include=Gtk-4.0 --pkg gtk4 \
		--library=gtk4-rlottie \
		--output $@ \
		$<

$(TYPELIB): $(GIR)
	mkdir -p $(@D)
	g-ir-compiler $< -o $@

$(VAPI): $(GIR)
	mkdir -p $(@D)
	vapigen \
		--pkg gtk4 \
		--library gtk4-rlottie \
		$< -d $(@D)
	echo gtk4 > $(@D)/gtk4-rlottie.deps

install: all
	cp -r inst/* $(DESTDIR)/

