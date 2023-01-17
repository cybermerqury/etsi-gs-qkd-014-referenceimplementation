# SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
# SPDX-License-Identifier: AGPL-3.0-only

.PHONY: certs clean

# NOTE: Use 'make VERBOSE=1 <target>' to show the commands being run
ifndef VERBOSE
.SILENT:
endif

.PRECIOUS: %.crt

LOG = output.log

ROOT_CRT = root.crt
ROOT_KEY  = root.key

SERIAL_NUM = 1

default: certs

certs: kme_001.pem sae_001.pem sae_002.pem

%.pem: %.crt
	printf "Generating pem file '%s'...\n" $@
	cat $(basename $^).key $^ > $@
	printf "Pem file '%s' generated\n" $@

%.crt: %.csr $(ROOT_CRT)
	printf "Generating certificate '%s' with serial '%d' signed using '%s'...\n" $@ $(SERIAL_NUM) $(ROOT_CRT)

	openssl x509 -req               \
 		-in $(word 1,$^)            \
		-CA $(ROOT_CRT)            \
 		-CAkey $(ROOT_KEY)          \
 		-set_serial $(SERIAL_NUM)   \
 		-days 365                   \
 		-extfile $(basename $@).ext \
 		-out $@                     \
		2>&1 | ts '%d-%m-%Y %H:%M:%S' >> $(LOG) 2>&1

	printf "Certificate '%s' generated\n" $@

	$(eval SERIAL_NUM=$(shell echo $$(($(SERIAL_NUM)+1))))

%.csr: %.ext
	printf "Generating certificate signing request for '%s'...\n" $@

	openssl req                                              \
 		-newkey rsa:4096                                     \
 		-nodes                                               \
 		-days 365                                            \
 		-subj "/C=MT/O=Acme Ltd/CN=$(basename $(notdir $@))" \
 		-keyout $(basename $@).key                           \
 		-out $@                                              \
		2>&1 | ts '%d-%m-%Y %H:%M:%S' >> $(LOG) 2>&1

	printf "Certificate '%s' signing request generated\n" $@

%.ext:
	echo "subjectAltName = DNS:localhost,IP:127.0.0.1" >> $@

kme.ext:
	echo "subjectAltName = DNS:localhost,IP:127.0.0.1" >> $@

$(ROOT_CRT):
	printf "Generating the root certificate '%s'...\n" $(ROOT_CRT)

	openssl req -x509                                            \
		-newkey rsa:4096                                         \
		-days 365                                                \
		-subj "/C=MT/CN=www.merqury.eu/O=Merqury\ Cybersecurity" \
		-keyout $(ROOT_KEY)                                      \
		-out $@                                                  \
		2>&1 | ts '%d-%m-%Y %H:%M:%S' >> $(LOG) 2>&1

	printf "Root certificate '%s' generated\n" $(ROOT_CRT)

clean:
	rm -f *.crt \
		  *.csr \
		  *.pem \
	 	  *.ext \
		  *.key \
		  $(LOG)

	printf "Certificates and output log removed\n"
