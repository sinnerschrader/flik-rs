import zeep
import json
from flask import Flask
from flask import jsonify
from flask import request

app = Flask(__name__)

# Login example: http://localhost:5000/BaseService/Login?username=&password=

@app.route("/<service>/<operation>")
def wsdlService(service, operation):
    wsdl = 'https://blueant.sinnerschrader.com/blueant/services/%s?wsdl' % service
    client = zeep.Client(wsdl=wsdl)
    my_args = {'password': 'Jim', 'username': 'France'}

    result = zeep.helpers.serialize_object(client.service[operation](**json.loads(json.dumps(request.args))))
    return jsonify(result)
