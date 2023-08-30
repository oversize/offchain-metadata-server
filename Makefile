# Some cargo helper


# Some python helper
venv:
	: # Create venv if it doesn't exist
	: # test -d venv || virtualenv -p python3 --no-site-packages venv
	test -d .venv || python3 -m venv .venv

install: venv
	: # Activate venv and install somthing inside
	(\
		source .venv/bin/activate;\
		pip install --upgrade pip\
		pip install requests\
	)

.PHONY: app
app:
	: # Run the app
	(\
		source .venv/bin/activate;\

	)

