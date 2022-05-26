perfect_clear: src/main.rs
	trunk build

install:
	test -n "${SITE_TARGET}"
	rm -r ${SITE_TARGET}/perfect_clear
	cp -r ./perfect_clear ${SITE_TARGET}/perfect_clear
