# Makefile for easily run maintenance commande

all: changelog

changelog:
	git cliff > CHANGELOG.md
