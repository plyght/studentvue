export interface StudentVueConfig {
  portalUrl: string;
  username: string;
  password: string;
}

export class StudentVueClient {
  private config: StudentVueConfig;

  constructor(config: StudentVueConfig) {
    this.config = config;
  }

  private createSoapEnvelope(
    username: string,
    password: string,
    serviceHandle: string,
    methodName: string,
    paramStr: string,
    multiWeb = false
  ): string {
    const requestType = multiWeb ? "ProcessWebServiceRequestMultiWeb" : "ProcessWebServiceRequest";
    
    return `<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
  <soap:Body>
    <${requestType} xmlns="http://edupoint.com/webservices/">
      <userID>${username}</userID>
      <password>${password}</password>
      <skipLoginLog>true</skipLoginLog>
      <parent>false</parent>
      <webServiceHandleName>${serviceHandle}</webServiceHandleName>
      <methodName>${methodName}</methodName>
      <paramStr>${paramStr}</paramStr>
    </${requestType}>
  </soap:Body>
</soap:Envelope>`;
  }

  private buildParamStr(params: Record<string, string>): string {
    const parts = Object.entries(params).map(
      ([key, value]) => `&lt;${key}&gt;${value}&lt;/${key}&gt;`
    );
    return `&lt;Parms&gt;${parts.join("")}&lt;/Parms&gt;`;
  }

  private async makeRequest(
    serviceHandle: string,
    methodName: string,
    params: Record<string, string>,
    multiWeb = false
  ): Promise<string> {
    const paramStr = this.buildParamStr(params);
    const soapRequest = this.createSoapEnvelope(
      this.config.username,
      this.config.password,
      serviceHandle,
      methodName,
      paramStr,
      multiWeb
    );

    const endpoint =
      serviceHandle === "HDInfoServices"
        ? `${this.config.portalUrl}/Service/HDInfoCommunication.asmx`
        : `${this.config.portalUrl}/Service/PXPCommunication.asmx`;

    const soapAction = multiWeb
      ? "http://edupoint.com/webservices/ProcessWebServiceRequestMultiWeb"
      : "http://edupoint.com/webservices/ProcessWebServiceRequest";

    const response = await fetch(endpoint, {
      method: "POST",
      headers: {
        "Content-Type": "text/xml; charset=utf-8",
        SOAPAction: soapAction,
        "User-Agent": "StudentVUE/8.0.26",
      },
      body: soapRequest,
    });

    if (!response.ok) {
      throw new Error(`Request failed with status: ${response.status}`);
    }

    const responseText = await response.text();
    return this.parseResponse(responseText);
  }

  private parseResponse(soapResponse: string): string {
    const match = soapResponse.match(
      /<ProcessWebServiceRequestResult>([\s\S]*?)<\/ProcessWebServiceRequestResult>/i
    ) || soapResponse.match(
      /<ProcessWebServiceRequestMultiWebResult>([\s\S]*?)<\/ProcessWebServiceRequestMultiWebResult>/i
    );

    if (!match) {
      throw new Error("Failed to parse SOAP response");
    }

    return match[1]
      .replace(/&lt;/g, "<")
      .replace(/&gt;/g, ">")
      .replace(/&amp;/g, "&")
      .replace(/&quot;/g, '"')
      .replace(/&apos;/g, "'");
  }

  async getStudentInfo(): Promise<string> {
    return this.makeRequest("PXPWebServices", "StudentInfo", { ChildIntID: "0" });
  }

  async getGradebook(reportPeriod?: number): Promise<string> {
    const params: Record<string, string> = { ChildIntID: "0" };
    if (reportPeriod !== undefined) {
      params.ReportPeriod = reportPeriod.toString();
    }
    return this.makeRequest("PXPWebServices", "Gradebook", params);
  }

  async getAttendance(): Promise<string> {
    return this.makeRequest("PXPWebServices", "Attendance", { ChildIntID: "0" });
  }

  async getMessages(): Promise<string> {
    return this.makeRequest("PXPWebServices", "GetPXPMessages", { childIntID: "0" });
  }

  async getCalendar(date: string): Promise<string> {
    return this.makeRequest("PXPWebServices", "StudentCalendar", {
      childIntID: "0",
      RequestDate: date,
    });
  }

  async getClassSchedule(termIndex?: number): Promise<string> {
    const params: Record<string, string> = { childIntID: "0" };
    if (termIndex !== undefined) {
      params.TermIndex = termIndex.toString();
    }
    return this.makeRequest("PXPWebServices", "StudentClassList", params);
  }

  async getSchoolInfo(): Promise<string> {
    return this.makeRequest("PXPWebServices", "StudentSchoolInfo", { childIntID: "0" });
  }

  async listDocuments(): Promise<string> {
    return this.makeRequest("PXPWebServices", "GetStudentDocumentInitialData", {
      childIntID: "0",
    });
  }

  async getDocument(documentGU: string): Promise<string> {
    return this.makeRequest("PXPWebServices", "GetContentOfAttachedDoc", {
      DocumentGU: documentGU,
    });
  }

  async listReportCards(): Promise<string> {
    return this.makeRequest("PXPWebServices", "GetReportCardInitialData", { childIntID: "0" });
  }

  async getReportCard(documentGU: string): Promise<string> {
    return this.makeRequest("PXPWebServices", "GetReportCardDocumentData", {
      DocumentGU: documentGU,
    });
  }

  async markMessageRead(messageId: string, messageType: string): Promise<string> {
    const paramStr = `&lt;Parms&gt;&lt;MessageListing ID="${messageId}" Type="${messageType}" MarkAsRead="true" /&gt;&lt;/Parms&gt;`;
    
    const soapRequest = this.createSoapEnvelope(
      this.config.username,
      this.config.password,
      "PXPWebServices",
      "UpdatePXPMessage",
      paramStr,
      true
    );

    const endpoint = `${this.config.portalUrl}/Service/PXPCommunication.asmx`;

    const response = await fetch(endpoint, {
      method: "POST",
      headers: {
        "Content-Type": "text/xml; charset=utf-8",
        SOAPAction: "http://edupoint.com/webservices/ProcessWebServiceRequestMultiWeb",
        "User-Agent": "StudentVUE/8.0.26",
      },
      body: soapRequest,
    });

    if (!response.ok) {
      throw new Error(`Request failed with status: ${response.status}`);
    }

    const responseText = await response.text();
    return this.parseResponse(responseText);
  }

  async getClassNotes(): Promise<string> {
    return this.makeRequest("PXPWebServices", "StudentHWNotes", { childIntID: "0" });
  }

  async getStudentHealthInfo(
    healthConditions: boolean,
    healthVisits: boolean,
    healthImmunizations: boolean
  ): Promise<string> {
    return this.makeRequest(
      "PXPWebServices",
      "StudentHealthInfo",
      {
        ChildIntID: "0",
        HealthConditions: healthConditions.toString(),
        HealthVisits: healthVisits.toString(),
        HealthImmunizations: healthImmunizations.toString(),
      },
      true
    );
  }

  async getDistrictsByZip(zipCode: string): Promise<string> {
    const params = {
      Key: "5E4B7859-B805-474B-A833-FDB15D205D40",
      MatchToDistrictZipCode: zipCode,
    };

    const paramStr = this.buildParamStr(params);
    const soapRequest = this.createSoapEnvelope(
      "EdupointDistrictInfo",
      "Edup01nt",
      "HDInfoServices",
      "GetMatchingDistrictList",
      paramStr,
      false
    );

    const response = await fetch("https://support.edupoint.com/Service/HDInfoCommunication.asmx", {
      method: "POST",
      headers: {
        "Content-Type": "text/xml; charset=utf-8",
        SOAPAction: "http://edupoint.com/webservices/ProcessWebServiceRequest",
      },
      body: soapRequest,
    });

    if (!response.ok) {
      throw new Error(`Request failed with status: ${response.status}`);
    }

    const responseText = await response.text();
    return this.parseResponse(responseText);
  }
}

