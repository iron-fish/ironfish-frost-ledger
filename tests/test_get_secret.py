from application_client.boilerplate_command_sender import (
    BoilerplateCommandSender,
    Errors,
)
from application_client.boilerplate_response_unpacker import unpack_get_secret_response
from ragger.bip import calculate_public_key_and_chaincode, CurveChoice
from ragger.error import ExceptionRAPDU
from ragger.navigator import NavInsID, NavIns
from utils import ROOT_SCREENSHOT_PATH


# In this test we check that the GET_PUBLIC_KEY works in non-confirmation mode
def test_get_secret_no_confirm(backend):
    client = BoilerplateCommandSender(backend)

    datum = client.get_secret().data

    print(f"{datum.hex()}")
    assert (
        datum.hex()
        == "00c3d2051e3b1fd517957d3cd1a4f819e19293e029cd9150fa3d246e1f92566a88baec7f07dac5cd3a6d251593821b6c0a2b04ecdf55452d30ce4fd8c8b54f7a31ad310c0e47ec1e6a26ca63e0c6070d2a9e851e19bd0c9dcd6a093b99ebcb63f2e66402e411804fed9453cdd8d56f8a450440048819e22e8443eafa2e15a7682543b1199502"
    )
