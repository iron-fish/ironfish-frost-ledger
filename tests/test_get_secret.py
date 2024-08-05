from application_client.boilerplate_command_sender import BoilerplateCommandSender, Errors
from application_client.boilerplate_response_unpacker import unpack_get_secret_response
from ragger.bip import calculate_public_key_and_chaincode, CurveChoice
from ragger.error import ExceptionRAPDU
from ragger.navigator import NavInsID, NavIns
from utils import ROOT_SCREENSHOT_PATH


# In this test we check that the GET_PUBLIC_KEY works in non-confirmation mode
def test_get_secret_no_confirm(backend):
    client = BoilerplateCommandSender(backend)
    round1_secret_package_hex = client.get_secret().data
    print(f'package {round1_secret_package_hex.hex()}')

    assert round1_secret_package_hex.hex() == ''