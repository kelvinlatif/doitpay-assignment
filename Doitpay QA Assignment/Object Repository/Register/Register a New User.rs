<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Register a New User</name>
   <tag></tag>
   <elementGuidId>4482de47-6206-4d5b-bba9-af2eae657088</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;id\&quot;: 1,\n    \&quot;name\&quot;: \&quot;E-Commerce User\&quot;,\n    \&quot;username\&quot;: \&quot;e-commerce-user\&quot;,\n    \&quot;email\&quot;: \&quot;e-commerce-user@test.com\&quot;,\n    \&quot;address\&quot;: {\n      \&quot;street\&quot;: \&quot;Kulas Light\&quot;,\n      \&quot;suite\&quot;: \&quot;Apt. 556\&quot;,\n      \&quot;city\&quot;: \&quot;Gwenborough\&quot;,\n      \&quot;zipcode\&quot;: \&quot;92998-3874\&quot;,\n      \&quot;geo\&quot;: {\n        \&quot;lat\&quot;: \&quot;-37.3159\&quot;,\n        \&quot;lng\&quot;: \&quot;81.1496\&quot;\n      }\n    },\n    \&quot;phone\&quot;: \&quot;6282356473829\&quot;,\n    \&quot;website\&quot;: \&quot;hildegard.org\&quot;,\n    \&quot;company\&quot;: {\n      \&quot;name\&quot;: \&quot;Romaguera-Crona\&quot;,\n      \&quot;catchPhrase\&quot;: \&quot;Multi-layered client-server neural-net\&quot;,\n      \&quot;bs\&quot;: \&quot;harness real-time e-markets\&quot;\n    }\n  }&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>841e4d10-3e2f-43ed-bb01-27705a3e2a22</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.0.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://jsonplaceholder.typicode.com/users</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
