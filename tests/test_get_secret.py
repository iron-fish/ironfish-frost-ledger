from application_client.boilerplate_command_sender import BoilerplateCommandSender, Errors
from application_client.boilerplate_response_unpacker import unpack_get_secret_response
from ragger.bip import calculate_public_key_and_chaincode, CurveChoice
from ragger.error import ExceptionRAPDU
from ragger.navigator import NavInsID, NavIns
from utils import ROOT_SCREENSHOT_PATH


# In this test we check that the GET_PUBLIC_KEY works in non-confirmation mode
def test_get_secret_no_confirm(backend):
    client = BoilerplateCommandSender(backend)
    data = []
    datum = client.get_secret().data
    index = 0
    while datum:
        data.append(datum)
        print(f'data{index} {datum.hex()}')
        datum = client.continue_apdu().data
        index += 1

    assert 1 == 0
