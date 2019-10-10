<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Reset Password</name>
   <tag></tag>
   <elementGuidId>8b6c0bd3-250c-4fba-8396-a5c230ed7b27</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;resetToken\&quot;:\&quot;eyJhbGciOiJIUzI1NiJ9.eyJqdGkiOiJzYWtzaGkuY2hhbmRlbEB0ZWNob2x1dGlvbi5jb20iLCJpYXQiOjE1NzA0NDA1MjksInN1YiI6IlRPS0VOLVZBTElEQVRJT04tU1VCSkVDVCIsImlzcyI6IkJZLUJPT1RDQU1QIiwiYXVkIjoidHJ1ZSIsImV4cCI6MTU3MDUyNjkyOX0.d-O8VV3_glq4j3oCjFPJfhDp0qGZowgTNu4BYfmYh_4\&quot;,\n    \&quot;password\&quot;:\&quot;sakshi\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/plain</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://34.68.136.212:8098/user/password/update</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


RequestObject request = WSResponseManager.getInstance().getCurrentRequest()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)


assertThat(response.getStatusCode()).isIn(Arrays.asList(200, 201, 202))</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
