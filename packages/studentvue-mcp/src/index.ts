#!/usr/bin/env node

import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import {
  CallToolRequestSchema,
  ListToolsRequestSchema,
  Tool,
} from "@modelcontextprotocol/sdk/types.js";
import { StudentVueClient } from "./studentvue-client.js";

const TOOLS: Tool[] = [
  {
    name: "get_student_info",
    description:
      "Retrieve student profile information including name, grade, school, contact info, and counselor details",
    inputSchema: {
      type: "object",
      properties: {},
      required: [],
    },
  },
  {
    name: "get_gradebook",
    description:
      "Retrieve current grades, assignments, and course information. Optionally specify a report period.",
    inputSchema: {
      type: "object",
      properties: {
        report_period: {
          type: "number",
          description: "Optional report period number to retrieve grades for",
        },
      },
    },
  },
  {
    name: "get_attendance",
    description: "Retrieve attendance records including absences, tardies, and reasons",
    inputSchema: {
      type: "object",
      properties: {},
      required: [],
    },
  },
  {
    name: "get_messages",
    description: "Retrieve inbox messages from teachers and administrators",
    inputSchema: {
      type: "object",
      properties: {},
      required: [],
    },
  },
  {
    name: "get_calendar",
    description: "Retrieve calendar events and upcoming assignments for a specific date",
    inputSchema: {
      type: "object",
      properties: {
        date: {
          type: "string",
          description: "Date in MM/DD/YYYY format",
        },
      },
      required: ["date"],
    },
  },
  {
    name: "get_class_schedule",
    description: "Retrieve class schedule with periods, teachers, and room assignments",
    inputSchema: {
      type: "object",
      properties: {
        term_index: {
          type: "number",
          description: "Optional term index to retrieve schedule for",
        },
      },
    },
  },
  {
    name: "get_school_info",
    description:
      "Retrieve school details including principal, address, phone, and contact information",
    inputSchema: {
      type: "object",
      properties: {},
      required: [],
    },
  },
  {
    name: "list_documents",
    description: "List all available student documents",
    inputSchema: {
      type: "object",
      properties: {},
      required: [],
    },
  },
  {
    name: "get_document",
    description: "Download a specific document by GUID. Returns base64-encoded document data.",
    inputSchema: {
      type: "object",
      properties: {
        document_gu: {
          type: "string",
          description: "The document GUID to retrieve",
        },
      },
      required: ["document_gu"],
    },
  },
  {
    name: "list_report_cards",
    description: "List available report cards by grading period",
    inputSchema: {
      type: "object",
      properties: {},
      required: [],
    },
  },
  {
    name: "get_report_card",
    description: "Download a specific report card by GUID. Returns base64-encoded PDF data.",
    inputSchema: {
      type: "object",
      properties: {
        document_gu: {
          type: "string",
          description: "The report card document GUID to retrieve",
        },
      },
      required: ["document_gu"],
    },
  },
  {
    name: "mark_message_read",
    description: "Mark a specific message as read",
    inputSchema: {
      type: "object",
      properties: {
        message_id: {
          type: "string",
          description: "The message ID to mark as read",
        },
        message_type: {
          type: "string",
          description: "The message type",
        },
      },
      required: ["message_id", "message_type"],
    },
  },
  {
    name: "get_class_notes",
    description: "Retrieve homework notes (availability depends on district configuration)",
    inputSchema: {
      type: "object",
      properties: {},
      required: [],
    },
  },
  {
    name: "get_student_health_info",
    description: "Retrieve student health information including conditions, visits, and immunizations",
    inputSchema: {
      type: "object",
      properties: {
        health_conditions: {
          type: "boolean",
          description: "Include health conditions",
          default: true,
        },
        health_visits: {
          type: "boolean",
          description: "Include health visits",
          default: true,
        },
        health_immunizations: {
          type: "boolean",
          description: "Include immunization records",
          default: true,
        },
      },
    },
  },
  {
    name: "get_districts_by_zip",
    description: "Search for school districts by ZIP code",
    inputSchema: {
      type: "object",
      properties: {
        zip_code: {
          type: "string",
          description: "5-digit ZIP code to search",
        },
      },
      required: ["zip_code"],
    },
  },
];

class StudentVueMCPServer {
  private server: Server;
  private client: StudentVueClient | null = null;

  constructor() {
    this.server = new Server(
      {
        name: "studentvue-mcp-server",
        version: "0.1.0",
      },
      {
        capabilities: {
          tools: {},
        },
      }
    );

    this.setupHandlers();
  }

  private setupHandlers() {
    this.server.setRequestHandler(ListToolsRequestSchema, async () => ({
      tools: TOOLS,
    }));

    this.server.setRequestHandler(CallToolRequestSchema, async (request) => {
      if (!this.client) {
        return {
          content: [
            {
              type: "text",
              text: "Error: StudentVue client not initialized. Please set STUDENTVUE_PORTAL, STUDENTVUE_USERNAME, and STUDENTVUE_PASSWORD environment variables.",
            },
          ],
        };
      }

      try {
        const { name, arguments: args } = request.params;

        let result: string;

        switch (name) {
          case "get_student_info":
            result = await this.client.getStudentInfo();
            break;

          case "get_gradebook":
            result = await this.client.getGradebook(args?.report_period as number | undefined);
            break;

          case "get_attendance":
            result = await this.client.getAttendance();
            break;

          case "get_messages":
            result = await this.client.getMessages();
            break;

          case "get_calendar":
            if (!args?.date) {
              throw new Error("date parameter is required");
            }
            result = await this.client.getCalendar(args.date as string);
            break;

          case "get_class_schedule":
            result = await this.client.getClassSchedule(args?.term_index as number | undefined);
            break;

          case "get_school_info":
            result = await this.client.getSchoolInfo();
            break;

          case "list_documents":
            result = await this.client.listDocuments();
            break;

          case "get_document":
            if (!args?.document_gu) {
              throw new Error("document_gu parameter is required");
            }
            result = await this.client.getDocument(args.document_gu as string);
            break;

          case "list_report_cards":
            result = await this.client.listReportCards();
            break;

          case "get_report_card":
            if (!args?.document_gu) {
              throw new Error("document_gu parameter is required");
            }
            result = await this.client.getReportCard(args.document_gu as string);
            break;

          case "mark_message_read":
            if (!args?.message_id || !args?.message_type) {
              throw new Error("message_id and message_type parameters are required");
            }
            result = await this.client.markMessageRead(
              args.message_id as string,
              args.message_type as string
            );
            break;

          case "get_class_notes":
            result = await this.client.getClassNotes();
            break;

          case "get_student_health_info":
            result = await this.client.getStudentHealthInfo(
              args?.health_conditions !== false,
              args?.health_visits !== false,
              args?.health_immunizations !== false
            );
            break;

          case "get_districts_by_zip":
            if (!args?.zip_code) {
              throw new Error("zip_code parameter is required");
            }
            result = await this.client.getDistrictsByZip(args.zip_code as string);
            break;

          default:
            throw new Error(`Unknown tool: ${name}`);
        }

        return {
          content: [
            {
              type: "text",
              text: result,
            },
          ],
        };
      } catch (error) {
        const errorMessage = error instanceof Error ? error.message : String(error);
        return {
          content: [
            {
              type: "text",
              text: `Error: ${errorMessage}`,
            },
          ],
          isError: true,
        };
      }
    });
  }

  private initializeClient() {
    const portalUrl = process.env.STUDENTVUE_PORTAL;
    const username = process.env.STUDENTVUE_USERNAME;
    const password = process.env.STUDENTVUE_PASSWORD;

    if (!portalUrl || !username || !password) {
      console.error(
        "Warning: StudentVue credentials not found in environment variables."
      );
      console.error("Please set STUDENTVUE_PORTAL, STUDENTVUE_USERNAME, and STUDENTVUE_PASSWORD.");
      return;
    }

    this.client = new StudentVueClient({
      portalUrl,
      username,
      password,
    });
  }

  async run() {
    this.initializeClient();

    const transport = new StdioServerTransport();
    await this.server.connect(transport);

    console.error("StudentVue MCP Server running on stdio");
  }
}

const server = new StudentVueMCPServer();
server.run().catch((error) => {
  console.error("Fatal error:", error);
  process.exit(1);
});

